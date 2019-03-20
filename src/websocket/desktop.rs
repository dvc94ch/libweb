use crate::{Error, Result};
use crate::websocket::{CloseCode, Message, Handler, Sender as WebSocketSender};
use std::borrow::Cow;

#[inline]
pub(crate) fn connect<T: Handler + 'static>(url: &str) -> Result<()> {
    ws::connect(url, |out| {
        WsHandler(T::new(WebSocketSender::new(Sender(out))))
    })?;
    Ok(())
}

pub struct Sender(ws::Sender);

impl Sender {
    #[inline]
    pub fn close_with_reason(
        &self,
        code: CloseCode,
        reason: impl Into<Cow<'static, str>>,
    ) -> Result<()> {
        let code: u16 = code.into();
        self.0.close_with_reason(ws::CloseCode::from(code), reason)?;
        Ok(())
    }

    #[inline]
    pub fn send(&self, msg: impl Into<Message>) -> Result<()> {
        let msg = match msg.into() {
            Message::Text(txt) => ws::Message::Text(txt),
            Message::Binary(bin) => ws::Message::Binary(bin),
        };
        self.0.send(msg)?;
        Ok(())
    }
}

struct WsHandler<T: Handler>(T);

impl<T: Handler> ws::Handler for WsHandler<T> {
    #[inline]
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        self.0.on_open().map_err(|err| to_ws_error(err, "on_open error"))
    }

    #[inline]
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        let msg = match msg {
            ws::Message::Text(txt) => Message::Text(txt),
            ws::Message::Binary(bin) => Message::Binary(bin),
        };
        self.0.on_message(msg).map_err(|err| to_ws_error(err, "on_message error"))
    }

    #[inline]
    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        let code: u16 = code.into();
        self.0.on_close(CloseCode::from(code), reason);
    }

    #[inline]
    fn on_error(&mut self, error: ws::Error) {
        self.0.on_error(error.into());
    }
}

#[inline]
fn to_ws_error(error: Error, msg: &'static str) -> ws::Error {
    ws::Error::new(ws::ErrorKind::Custom(Box::new(error.compat())), msg)
}
