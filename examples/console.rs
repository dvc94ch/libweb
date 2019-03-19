#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use libweb::console_log;

fn main() {
    let world = "world";
    console_log!("hello {}", world);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run() {
    main()
}
