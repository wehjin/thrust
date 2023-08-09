use wasm_bindgen::prelude::*;
use crate::three::BufferGeometry;

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
	#[derive(Clone)]
	#[wasm_bindgen(extends = BufferGeometry)]
	pub type CylinderGeometry;
	#[wasm_bindgen(constructor)]
	pub fn new(radius_top: f64, radius_bottom: f64, height: f64) -> CylinderGeometry;
}
