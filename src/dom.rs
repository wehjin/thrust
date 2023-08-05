use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

pub type Error = JsValue;

pub fn animate_frames(mut frame: impl FnMut() -> bool + 'static) {
	fn request_animation_frame(f: &Closure<dyn FnMut()>) {
		window().request_animation_frame(f.as_ref().unchecked_ref()).expect("should register `requestAnimationFrame` OK");
	}

	let f = Rc::new(RefCell::new(None));
	let g = f.clone();
	*g.borrow_mut() = Some(Closure::new(move || {
		let repeat = frame();
		if repeat {
			request_animation_frame(f.borrow().as_ref().unwrap());
		} else {
			f.borrow_mut().take();
		}
	}));
	request_animation_frame(g.borrow().as_ref().unwrap());
}


pub(crate) fn window() -> web_sys::Window {
	web_sys::window().expect("no global `window` exists")
}

fn document() -> web_sys::Document {
	window()
		.document()
		.expect("should have a document on window")
}

pub(crate) fn body() -> web_sys::HtmlElement {
	document().body().expect("document should have a body")
}
