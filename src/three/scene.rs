use wasm_bindgen::prelude::*;
use crate::three::{Object3D};

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
	#[derive(Clone, Default)]
	#[wasm_bindgen(extends = Object3D)]
	pub type Scene;
	#[wasm_bindgen(constructor)]
	pub fn new() -> Scene;
}
