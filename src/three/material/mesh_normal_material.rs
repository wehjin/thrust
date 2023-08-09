use wasm_bindgen::prelude::*;
use crate::three::{Material};

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
	#[derive(Clone)]
	#[wasm_bindgen(extends = Material)]
	pub type MeshNormalMaterial;
	#[wasm_bindgen(constructor)]
	pub fn new() -> MeshNormalMaterial;
}
