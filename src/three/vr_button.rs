use wasm_bindgen::prelude::*;
use web_sys::HtmlButtonElement;
use crate::three::WebGLRenderer;

#[wasm_bindgen(module = "/vendor/three/VRButton.js")]
extern "C" {
	#[wasm_bindgen(js_namespace = VRButton, js_name = createButton)]
	pub fn create_button(renderer: &WebGLRenderer) -> HtmlButtonElement;
}
