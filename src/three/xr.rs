use wasm_bindgen::prelude::*;
use crate::three::Group;

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
	pub type WebXRManager;
	#[wasm_bindgen(method, setter)]
	pub fn set_enabled(this: &WebXRManager, val: bool);
	#[wasm_bindgen(method, js_name = getController)]
	pub fn get_controller(this: &WebXRManager, index: i32) -> Group;

	#[wasm_bindgen(method, js_name = getHand)]
	pub fn get_hand(this: &WebXRManager, index: i32) -> Group;
}

