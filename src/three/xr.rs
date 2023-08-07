use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
	pub type WebXRManager;
	#[wasm_bindgen(method, setter)]
	pub fn set_enabled(this: &WebXRManager, val: bool);
}
