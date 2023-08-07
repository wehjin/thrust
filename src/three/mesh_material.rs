use wasm_bindgen::prelude::*;
use crate::three::Color;

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
	pub type MeshNormalMaterial;
	#[wasm_bindgen(constructor)]
	pub fn new() -> MeshNormalMaterial;

	pub type MeshBasicMaterial;
	#[wasm_bindgen(constructor)]
	pub fn new() -> MeshBasicMaterial;
	#[wasm_bindgen(method, getter)]
	pub fn color(this: &MeshBasicMaterial) -> Color;
}
