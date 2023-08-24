use wasm_bindgen::prelude::*;
use crate::three::{BufferGeometry, Euler, Material, Object3D, Vector3};

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
	#[wasm_bindgen(extends = Object3D)]
	pub type Mesh;
	#[wasm_bindgen(constructor)]
	pub fn new(geometry: &BufferGeometry, material: &Material) -> Mesh;
	#[wasm_bindgen(method, getter)]
	pub fn rotation(this: &Mesh) -> Euler;
	#[wasm_bindgen(method, getter)]
	pub fn position(this: &Mesh) -> Vector3;

	#[wasm_bindgen(method, getter)]
	pub fn material(this: &Mesh) -> Material;
}
