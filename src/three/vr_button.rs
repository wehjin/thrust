use wasm_bindgen::prelude::*;
use web_sys::HtmlButtonElement;
use crate::three::WebGLRenderer;

#[wasm_bindgen(module = "/node_modules/three/examples/jsm/webxr/VRButton.js")]
extern "C" {
	#[wasm_bindgen(js_namespace = VRButton, js_name = createButton)]
	pub fn create_button(renderer: &WebGLRenderer) -> HtmlButtonElement;
}
