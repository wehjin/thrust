use wasm_bindgen::prelude::*;

fn main() {
	wasm_logger::init(wasm_logger::Config::default());
	log::info!("Boo!");
}

#[wasm_bindgen]
extern "C" {
	fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
	alert(&format!("Hello, {}!", name));
}
