use wasm_bindgen::prelude::*;
use crate::three::BufferGeometry;

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
	#[derive(Clone)]
	#[wasm_bindgen(extends = BufferGeometry)]
	pub type BoxGeometry;
	#[wasm_bindgen(constructor)]
	pub fn new(width: f64, height: f64, depth: f64) -> BoxGeometry;
}
