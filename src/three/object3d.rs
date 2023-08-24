use wasm_bindgen::prelude::*;
use crate::three::EventDispatcher;

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
	#[derive(Clone, Default)]
	#[wasm_bindgen(extends = EventDispatcher)]
	pub type Object3D;

	#[wasm_bindgen(method)]
	pub fn add(this: &Object3D, object: &Object3D) -> Object3D;
}
