use failure::bail;
use libweb::*;
use libweb::websocket::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

struct PongHandler(Sender);

impl PongHandler {
    fn ping(&self) -> Result<()> {
        self.0.send("ping")
    }

    fn pong(&self) -> Result<()> {
        self.0.send(&b"pong"[..])
    }

    fn close(&self) -> Result<()> {
        self.0.close(CloseCode::Normal)
    }
}

impl Handler for PongHandler {
    fn new(out: Sender) -> Self {
        PongHandler(out)
    }

    fn on_open(&mut self) -> Result<()> {
        self.ping()
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        console_log!("received message {:?}", msg);
        match msg {
            Message::Text(ref text) => {
                if text == "ping" {
                    self.pong()?;
                } else {
                    bail!("Expected ping");
                }
            }
            Message::Binary(ref bin) => {
                if bin == b"pong" {
                    self.close()?;
                } else {
                    bail!("Expected pong");
                }
            }
        }
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        console_log!("Connection closing due to ({:?}) {}", code, reason);
    }

    fn on_error(&mut self, error: Error) {
        console_error!("{:?}", error);
    }
}

fn main() -> Result<()> {
    connect::<PongHandler>("ws://127.0.0.1:3012")?;
    Ok(())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run() {
    console_error_panic_hook::set_once();
    main().unwrap();
}
