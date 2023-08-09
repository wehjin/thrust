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
		scene.add_camera(&camera);
		let renderer = three::WebGLRenderer::new();
		renderer.set_pixel_ratio(window.as_dom_window().device_pixel_ratio());
		renderer.set_size(width as isize, height as isize, false);
		renderer.xr().set_enabled(true);
		window.add_resize_listener_with_forget({
			let camera = camera.clone();
			let renderer = renderer.clone();
			move |_event| {
				let window = Window::connect();
				let (width, height) = window.inner_size();
				camera.set_aspect(width / height);
				camera.update_projection_matrix();
				renderer.set_size(width as isize, height as isize, false);
			}
		});
		window.body().append_child(&renderer.dom_element()).expect("body must append renderer's dom element");
		window.body().append_child(&three::create_button(&renderer)).expect("append button");
		Self { renderer, camera, scene }
	}
	pub fn camera(&self) -> &three::PerspectiveCamera { &self.camera }
	pub fn scene(&self) -> &three::Scene { &self.scene }
	pub fn as_three(&self) -> &three::WebGLRenderer { &self.renderer }
	pub fn set_animation_loop_with_forget(&self, f: impl FnMut(f64) + 'static) {
		let closure = Closure::<dyn FnMut(f64)>::new(f);
		self.renderer.set_animation_loop(&closure);
		closure.forget();
	}
}
