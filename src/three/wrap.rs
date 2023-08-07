use wasm_bindgen::prelude::*;
use crate::three::WebGLRenderer;

pub fn set_animation_loop_with_forget(renderer: &WebGLRenderer, f: impl FnMut(f64) + 'static) {
	let closure = Closure::<dyn FnMut(f64)>::new(f);
	renderer.set_animation_loop(&closure);
	closure.forget();
}
