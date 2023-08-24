use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
	#[derive(Clone, Default)]
	pub type EventDispatcher;

	#[wasm_bindgen(method, js_name = addEventListener)]
	pub fn add_event_listener(this: &EventDispatcher, event_type: &str, listener: &Closure<dyn FnMut()>);
}
