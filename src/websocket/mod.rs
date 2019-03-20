//! The WebSocket API
use crate::{Error, Result};
use log::*;
use self::CloseCode::*;
use std::borrow::Cow;

#[cfg(target_arch = "wasm32")]
mod browser;
#[cfg(not(target_arch = "wasm32"))]
mod desktop;

#[cfg(target_arch = "wasm32")]
type InnerSender = browser::Sender;
#[cfg(not(target_arch = "wasm32"))]
type InnerSender = desktop::Sender;

/// Create a new WebSocket connection to url.
pub fn connect<T: Handler + 'static>(url: &str) -> Result<()> {
    #[cfg(target_arch = "wasm32")]
    browser::connect::<T>(url)?;
    #[cfg(not(target_arch = "wasm32"))]
    desktop::connect::<T>(url)?;
    Ok(())
}

/// A representation of the output of the WebSocket connection. Use this to send
/// messages to the other endpoint.
pub struct Sender {
    inner: InnerSender,
}

impl Sender {
    #[inline]
    pub(crate) fn new(inner: InnerSender) -> Self {
        Sender {
            inner,
        }
    }

    /// Send a close code to the other endpoint.
    #[inline]
    pub fn close(&self, code: CloseCode) -> Result<()> {
        self.close_with_reason(code, "")
    }

    /// Send a close code and provide a descriptive reason for closing.
    #[inline]
    pub fn close_with_reason(
        &self,
        code: CloseCode,
        reason: impl Into<Cow<'static, str>>,
    ) -> Result<()> {
        #[cfg(target_arch = "wasm32")]
        self.inner.borrow().close_with_reason(code, reason)?;
        #[cfg(not(target_arch = "wasm32"))]
        self.inner.close_with_reason(code, reason)?;
        Ok(())
    }

    /// Send a message over the connection.
    #[inline]
    pub fn send(&self, msg: impl Into<Message>) -> Result<()> {
        #[cfg(target_arch = "wasm32")]
        self.inner.borrow().send(msg)?;
        #[cfg(not(target_arch = "wasm32"))]
        self.inner.send(msg)?;
        Ok(())
    }
}

/// Implementing this trait provides the business logic of the WebSocket
/// application.
pub trait Handler {
    /// Creates a new handler
    fn new(sender: Sender) -> Self;

    /// Called when the WebSocket handshake is successful and the connection is
    /// open for sending and receiving messages.
    fn on_open(&mut self) -> Result<()> {
        debug!("Connection open");
        Ok(())
    }

    /// Called on incoming messages.
    fn on_message(&mut self, msg: Message) -> Result<()> {
        debug!("Received message {:?}", msg);
        Ok(())
    }

    /// Called any time this endpoint receives a close control frame.
    /// This may be because the other endpoint is initiating a closing
    /// handshake, or it may be the other endpoint confirming the handshake
    /// initiated by this endpoint.
    fn on_close(&mut self, code: CloseCode, reason: &str) {
        debug!("Connection closing due to ({:?}) {}", code, reason);
    }

    /// Called when an error occurs on the WebSocket.
    fn on_error(&mut self, err: Error) {
        error!("{:?}", err);
        if !log_enabled!(Level::Error) {
            println!("Encountered an error: {}", err);
            println!("Enable a logger to see more information.");
        }
    }
}

/// An enum representing the various forms of a WebSocket message.
#[derive(Debug)]
pub enum Message {
    /// A text WebSocket message
    Text(String),
    /// A binary WebSocket message
    Binary(Vec<u8>),
}

impl Message {
    /// Create a new text WebSocket message from a stringable.
    pub fn text<S>(string: S) -> Message
    where
        S: Into<String>,
    {
        Message::Text(string.into())
    }

    /// Create a new binary WebSocket message by converting to Vec<u8>.
    pub fn binary<B>(bin: B) -> Message
    where
        B: Into<Vec<u8>>,
    {
        Message::Binary(bin.into())
    }
}

impl From<String> for Message {
    fn from(string: String) -> Message {
        Message::text(string)
    }
}

impl<'a> From<&'a str> for Message {
    fn from(string: &'a str) -> Message {
        Message::text(string)
    }
}

impl<'a> From<&'a [u8]> for Message {
    fn from(data: &'a [u8]) -> Message {
        Message::binary(data)
    }
}

impl From<Vec<u8>> for Message {
    fn from(data: Vec<u8>) -> Message {
        Message::binary(data)
    }
}

/// Status code used to indicate why an endpoint is closing the WebSocket connection.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum CloseCode {
    /// Indicates a normal closure, meaning that the purpose for
    /// which the connection was established has been fulfilled.
    Normal,
    /// Indicates that an endpoint is "going away", such as a server
    /// going down or a browser having navigated away from a page.
    Away,
    /// Indicates that an endpoint is terminating the connection due
    /// to a protocol error.
    Protocol,
    /// Indicates that an endpoint is terminating the connection
    /// because it has received a type of data it cannot accept (e.g., an
    /// endpoint that understands only text data MAY send this if it
    /// receives a binary message).
    Unsupported,
    /// Indicates that no status code was included in a closing frame. This
    /// close code makes it possible to use a single method, `on_close` to
    /// handle even cases where no close code was provided.
    Status,
    /// Indicates an abnormal closure. If the abnormal closure was due to an
    /// error, this close code will not be used. Instead, the `on_error` method
    /// of the handler will be called with the error. However, if the connection
    /// is simply dropped, without an error, this close code will be sent to the
    /// handler.
    Abnormal,
    /// Indicates that an endpoint is terminating the connection
    /// because it has received data within a message that was not
    /// consistent with the type of the message (e.g., non-UTF-8 [RFC3629]
    /// data within a text message).
    Invalid,
    /// Indicates that an endpoint is terminating the connection
    /// because it has received a message that violates its policy.  This
    /// is a generic status code that can be returned when there is no
    /// other more suitable status code (e.g., Unsupported or Size) or if there
    /// is a need to hide specific details about the policy.
    Policy,
    /// Indicates that an endpoint is terminating the connection
    /// because it has received a message that is too big for it to
    /// process.
    Size,
    /// Indicates that an endpoint (client) is terminating the
    /// connection because it has expected the server to negotiate one or
    /// more extension, but the server didn't return them in the response
    /// message of the WebSocket handshake.  The list of extensions that
    /// are needed should be given as the reason for closing.
    /// Note that this status code is not used by the server, because it
    /// can fail the WebSocket handshake instead.
    Extension,
    /// Indicates that a server is terminating the connection because
    /// it encountered an unexpected condition that prevented it from
    /// fulfilling the request.
    Error,
    /// Indicates that the server is restarting. A client may choose to reconnect,
    /// and if it does, it should use a randomized delay of 5-30 seconds between attempts.
    Restart,
    /// Indicates that the server is overloaded and the client should either connect
    /// to a different IP (when multiple targets exist), or reconnect to the same IP
    /// when a user has performed an action.
    Again,
    #[doc(hidden)]
    Tls,
    #[doc(hidden)]
    Empty,
    #[doc(hidden)]
    Other(u16),
}

impl Into<u16> for CloseCode {
    fn into(self) -> u16 {
        match self {
            Normal => 1000,
            Away => 1001,
            Protocol => 1002,
            Unsupported => 1003,
            Status => 1005,
            Abnormal => 1006,
            Invalid => 1007,
            Policy => 1008,
            Size => 1009,
            Extension => 1010,
            Error => 1011,
            Restart => 1012,
            Again => 1013,
            Tls => 1015,
            Empty => 0,
            Other(code) => code,
        }
    }
}

impl From<u16> for CloseCode {
    fn from(code: u16) -> CloseCode {
        match code {
            1000 => Normal,
            1001 => Away,
            1002 => Protocol,
            1003 => Unsupported,
            1005 => Status,
            1006 => Abnormal,
            1007 => Invalid,
            1008 => Policy,
            1009 => Size,
            1010 => Extension,
            1011 => Error,
            1012 => Restart,
            1013 => Again,
            1015 => Tls,
            0 => Empty,
            _ => Other(code),
        }
    }
}
