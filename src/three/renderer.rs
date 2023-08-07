use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use crate::three::{PerspectiveCamera, Scene, WebXRManager};

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
	pub type WebGLRenderer;
	#[wasm_bindgen(constructor)]
	pub fn new() -> WebGLRenderer;
	#[wasm_bindgen(method, js_name = "setSize")]
	pub fn set_size(this: &WebGLRenderer, width: isize, height: isize, update_style: bool);
	#[wasm_bindgen(method, getter, js_name = "domElement")]
	pub fn dom_element(this: &WebGLRenderer) -> HtmlElement;
	#[wasm_bindgen(method, getter)]
	pub fn xr(this: &WebGLRenderer) -> WebXRManager;
	#[wasm_bindgen(method)]
	pub fn render(this: &WebGLRenderer, scene: &Scene, camera: &PerspectiveCamera);
	#[wasm_bindgen(method, js_name = "setAnimationLoop")]
	pub fn set_animation_loop(this: &WebGLRenderer, f: &Closure<dyn FnMut(f64)>);
}
