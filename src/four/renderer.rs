use wasm_bindgen::prelude::Closure;
use crate::dom::Window;
use crate::three;

#[derive(Clone, Default)]
pub struct Renderer {
	renderer: three::WebGLRenderer,
	camera: three::PerspectiveCamera,
	scene: three::Scene,
}

impl Renderer {
	pub fn new(window: &Window) -> Self {
		let (width, height) = window.inner_size();
		let camera = three::immersive_camera(width, height);
		let scene = three::Scene::new();
		scene.add(&camera.as_ref());
		let renderer = three::WebGLRenderer::new();
		renderer.set_pixel_ratio(window.as_dom_window().device_pixel_ratio());
		renderer.set_size(width as isize, height as isize, false);
		renderer.xr().set_enabled(true);
		window.body().append_child(&renderer.dom_element()).expect("body must append renderer's dom element");
		window.body().append_child(&three::create_button(&renderer)).expect("append button");
		window.add_and_forget_resize_listener({
			let renderer = renderer.clone();
			let camera = camera.clone();
			move |_event| {
				let window = Window::connect();
				let (width, height) = window.inner_size();
				camera.set_aspect(width / height);
				camera.update_projection_matrix();
				renderer.set_size(width as isize, height as isize, false);
			}
		});
		Self { renderer, camera, scene }
	}
	pub fn camera(&self) -> &three::PerspectiveCamera { &self.camera }
	pub fn scene(&self) -> &three::Scene { &self.scene }
	pub fn as_three(&self) -> &three::WebGLRenderer { &self.renderer }
	pub fn set_and_forget_animation_loop(&self, f: impl FnMut(f64) + 'static) {
		let closure = Closure::<dyn FnMut(f64)>::new(f);
		self.renderer.set_animation_loop(&closure);
		closure.forget();
	}
	pub fn dom_element(&self) -> web_sys::HtmlElement { self.renderer.dom_element() }
	pub fn xr(&self) -> three::WebXRManager { self.renderer.xr() }
}
