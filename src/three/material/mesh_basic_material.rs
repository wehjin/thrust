use wasm_bindgen::prelude::*;
use crate::three::{Color, Material};

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
	#[derive(Clone)]
	#[wasm_bindgen(extends = Material)]
	pub type MeshBasicMaterial;
	#[wasm_bindgen(constructor)]
	pub fn new() -> MeshBasicMaterial;
	#[wasm_bindgen(method, getter)]
	pub fn color(this: &MeshBasicMaterial) -> Color;
}
