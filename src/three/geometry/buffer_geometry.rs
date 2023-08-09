use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
	#[derive(Clone)]
	pub type BufferGeometry;
	#[wasm_bindgen(constructor)]
	pub fn new(radius_top: f64, radius_bottom: f64, height: f64) -> BufferGeometry;
}
