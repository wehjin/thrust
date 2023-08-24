use wasm_bindgen::prelude::*;
use crate::three::EventDispatcher;
use crate::three::object3d::Object3D;

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
	#[wasm_bindgen(extends = Object3D, extends = EventDispatcher)]
	pub type Group;
}

