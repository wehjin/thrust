use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

pub type Error = JsValue;

pub mod console {
	use wasm_bindgen::prelude::*;

	#[wasm_bindgen]
	extern "C" {
		#[wasm_bindgen(js_namespace = console)]
		pub fn log(s: &str);
	}
}

pub struct Window(web_sys::Window);

impl Window {
	pub fn connect() -> Self {
		let window = web_sys::window().expect("no global `window` exists");
		Window(window)
	}
	pub fn as_dom_window(&self) -> &web_sys::Window { &self.0 }
	pub fn inner_size(&self) -> (f64, f64) {
		let (width, height) = self.js_inner_size();
		(width.as_f64().unwrap(), height.as_f64().unwrap())
	}
	pub fn body(&self) -> web_sys::HtmlElement {
		self.document().body().expect("document should have a body")
	}
	pub fn document(&self) -> web_sys::Document {
		self.0.document().expect("should have a document on window")
	}
	pub fn animate_frames(&self, mut frame: impl FnMut() -> FrameNext + 'static) {
		let setup_cell = Rc::new(RefCell::new(None));
		{
			let cleanup_cell = setup_cell.clone();
			*setup_cell.borrow_mut() = Some(Closure::new(move || {
				let next_frame = frame();
				match next_frame {
					FrameNext::Repeat => {
						Window::connect().js_request_animation_frame(cleanup_cell.borrow().as_ref().unwrap());
					}
					FrameNext::Stop => {
						cleanup_cell.borrow_mut().take();
					}
				}
			}));
		}
		self.js_request_animation_frame(setup_cell.borrow().as_ref().unwrap());
	}
	pub fn add_and_forget_resize_listener(&self, f: impl FnMut(web_sys::UiEvent) + 'static) {
		let closure = Closure::<dyn FnMut(_)>::new(f);
		self.0.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref()).expect("add resize listener");
		closure.forget();
	}

	fn js_request_animation_frame(&self, f: &Closure<dyn FnMut()>) {
		self.0.request_animation_frame(f.as_ref().unchecked_ref()).expect("should register `requestAnimationFrame` OK");
	}
	fn js_inner_size(&self) -> (JsValue, JsValue) {
		let (width, height): (JsValue, JsValue) = (
			self.0.inner_width().expect("window must have inner width"),
			self.0.inner_height().expect("window must have inner height")
		);
		(width, height)
	}
	pub fn add_and_forget_keydown_listener(&self, f: impl FnMut(web_sys::KeyboardEvent) + 'static) {
		let closure = Closure::<dyn FnMut(_)>::new(f);
		self.body().add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).expect("add key-down listener");
		closure.forget();
	}
}

pub enum FrameNext { Repeat, Stop }
