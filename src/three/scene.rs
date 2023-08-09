use wasm_bindgen::prelude::*;
use crate::three::{Mesh, PerspectiveCamera};

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
	#[derive(Clone, Default)]
	pub type Scene;
	#[wasm_bindgen(constructor)]
	pub fn new() -> Scene;
	#[wasm_bindgen(method)]
	pub fn add(this: &Scene, mesh: &Mesh);
	#[wasm_bindgen(method, js_name = add)]
	pub fn add_camera(this: &Scene, camera: &PerspectiveCamera);
}
