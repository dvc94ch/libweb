#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use libweb::console_log;

#[cfg(not(target_arch = "wasm32"))]
mod server {
    use super::*;
    use ws::listen;

    pub fn main() {
        // Setup logging
        env_logger::init();

        // Listen on an address and call the closure for each connection
        if let Err(error) = listen("127.0.0.1:3012", |out| {
            console_log!("Server got connection");

            // The handler needs to take ownership of out, so we use move
            move |msg| {
                // Handle messages received on this connection
                console_log!("Server got message '{}'. ", msg);

                // Use the out channel to send messages back
                out.send(msg)
            }
        }) {
            // Inform the user of failure
            console_log!("Failed to create WebSocket due to {:?}", error);
        }
    }
}

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    server::main();
    #[cfg(target_arch = "wasm32")]
    console_log!("web socket server can't work in a browser");
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run() {
    console_error_panic_hook::set_once();
    main()
}
