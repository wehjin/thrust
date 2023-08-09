use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
	#[derive(Clone)]
	pub type Material;
}
