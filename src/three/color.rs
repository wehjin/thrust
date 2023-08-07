use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
	pub type Color;
	#[wasm_bindgen(constructor)]
	pub fn new(r: f64, g: f64, b: f64) -> Color;
	#[wasm_bindgen(method)]
	pub fn set(this: &Color, r: f64, g: f64, b: f64);
}
