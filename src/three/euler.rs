use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
	pub type Euler;
	#[wasm_bindgen(method, getter)]
	pub fn x(this: &Euler) -> JsValue;
	#[wasm_bindgen(method, getter)]
	pub fn y(this: &Euler) -> JsValue;
	#[wasm_bindgen(method)]
	pub fn set(this: &Euler, x: f64, y: f64, z: f64);
}
