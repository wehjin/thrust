use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
	pub type Vector3;
	#[wasm_bindgen(constructor)]
	pub fn new(x: f64, y: f64, z: f64) -> Vector3;
	#[wasm_bindgen(method, setter)]
	pub fn set_z(this: &Vector3, val: f64);
	#[wasm_bindgen(method, js_name = set)]
	pub fn set_xyz(this: &Vector3, x: f64, y: f64, z: f64);
}
