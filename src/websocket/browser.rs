use crate::{js_value_to_error, Result};
use crate::websocket::{CloseCode, Message, Handler, Sender as WebSocketSender};
use js_sys::Uint8Array;
use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub type Sender = Rc<RefCell<WebSocket>>;

#[inline]
pub(crate) fn connect<T: Handler + 'static>(url: &str) -> Result<()> {
    let socket = Rc::new(RefCell::new(WebSocket::new(url)?));
    let sender = WebSocketSender::new(Rc::clone(&socket));
    let handler = T::new(sender);
    socket.borrow_mut().set_handler(handler);
    Ok(())
}

pub struct WebSocket {
    socket: web_sys::WebSocket,
}

impl WebSocket {
    #[inline]
    pub fn new(url: &str) -> Result<Self> {
        let socket = web_sys::WebSocket::new(url)
            .map_err(js_value_to_error)?;
        socket.set_binary_type(web_sys::BinaryType::Arraybuffer);
        Ok(WebSocket { socket })
    }

    #[inline]
    pub fn close_with_reason(
        &self,
        code: CloseCode,
        reason: impl Into<Cow<'static, str>>,
    ) -> Result<()> {
        self.socket
            .close_with_code_and_reason(code.into(), &reason.into())
            .map_err(js_value_to_error)
    }

    #[inline]
    pub fn send(&self, msg: impl Into<Message>) -> Result<()> {
        match msg.into() {
            Message::Text(txt) => {
                self.socket
                    .send_with_str(&txt)
                    .map_err(js_value_to_error)?
            },
            Message::Binary(bin) => {
                let arr = unsafe { Uint8Array::view(&bin) };
                let arr = Uint8Array::new(&arr).buffer();
                self.socket
                    .send_with_array_buffer(&arr)
                    .map_err(js_value_to_error)?
            }
        };
        Ok(())
    }

    pub fn set_handler(&self, handler: impl Handler + 'static) {
        let handler = Rc::new(RefCell::new(Box::new(handler)));

        let handler2 = Rc::clone(&handler);
        let onopen: Closure<FnMut(JsValue)> =
            Closure::new(move |_event: JsValue| {
                let result = handler2.borrow_mut().on_open();
                match result {
                    Ok(()) => (),
                    Err(err) => handler2.borrow_mut().on_error(err),
                }
            });
        self.socket.set_onopen(Some(onopen.as_ref().unchecked_ref()));
        onopen.forget();

        let handler2 = Rc::clone(&handler);
        let onmessage: Closure<FnMut(JsValue)> =
            Closure::new(move |event: JsValue| {
                let data = js_sys::Reflect::get(&event, &JsValue::from("data"))
                    .unwrap();
                let message = if data.is_string() {
                    Message::text(data.as_string().unwrap())
                } else {
                    let buffer = js_sys::Uint8Array::new(&data);
                    let mut bin = vec![0; buffer.length() as usize];
                    buffer.copy_to(&mut bin[..]);
                    Message::binary(bin)
                };
                let result = handler2.borrow_mut().on_message(message);
                match result {
                    Ok(()) => (),
                    Err(err) => handler2.borrow_mut().on_error(err),
                }
            });
        self.socket.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
        onmessage.forget();

        let handler2 = Rc::clone(&handler);
        let onclose: Closure<FnMut(JsValue)> =
            Closure::new(move |event: JsValue| {
                let code = js_sys::Reflect::get(&event, &JsValue::from("code"))
                    .unwrap()
                    .as_f64()
                    .unwrap();
                let code = CloseCode::from(code as u16);
                let reason = js_sys::Reflect::get(&event, &JsValue::from("reason"))
                    .unwrap()
                    .as_string()
                    .unwrap();
                handler2.borrow_mut().on_close(code, &reason);
            });
        self.socket.set_onclose(Some(onclose.as_ref().unchecked_ref()));
        onclose.forget();

        let onerror: Closure<FnMut(JsValue)> =
            Closure::new(move |event: JsValue| {
                let error = js_value_to_error(event);
                handler.borrow_mut().on_error(error);
            });
        self.socket.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        onerror.forget();
    }
}
