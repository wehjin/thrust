pub mod three;
pub mod dom;
pub mod basics;
pub mod builders;

use wasm_bindgen::prelude::*;
use dom::Window;
use three::{PerspectiveCamera, WebGLRenderer};
use crate::builders::VarSceneBuilder;
use crate::dom::{Error, FrameNext};


fn main() -> Result<(), Error> {
	wasm_logger::init(wasm_logger::Config::default());
	log::info!("Boo!");

	let (mut x, mut y) = (0.0, 0.0);


	const GEOMETRY: &'static str = "geometry";
	const MATERIAL: &'static str = "material";
	const CUBE: &'static str = "cube";
	const ROTATION: &'static str = "rotation";
	let var_scene = {
		let mut builder = VarSceneBuilder::new();
		builder.add_val_box_geo(GEOMETRY, (2.0, 1.0, 1.0).into());
		builder.add_val_mesh_basic_mat(MATERIAL, (255.0, 0.0, 0.0).into());
		builder.start_var_mesh(CUBE, GEOMETRY, MATERIAL);
		builder.add_var_mesh_rot(ROTATION, (x, y, 0.0).into());
		builder.end_var_mesh();
		builder.to_var_scene()
	};
	let window = Window::connect();
	let (camera, renderer) = {
		let (inner_width, inner_height) = window.inner_size();
		let camera = create_camera(5.0, inner_width, inner_height);
		let renderer = attach_renderer(&window, WebGLRenderer::new(), inner_width, inner_height);
		(camera, renderer)
	};
	window.animate_frames(move || {
		let increment = 0.01;
		(x, y) = (x + increment, y + 3.0 * increment);
		var_scene.set_mesh_rot_val(CUBE, ROTATION, (x, y, 0.0).into());
		renderer.render(var_scene.as_three_scene(), &camera);
		FrameNext::Repeat
	});
	Ok(())
}

fn create_camera(z: f64, inner_width: f64, inner_height: f64) -> PerspectiveCamera {
	let camera = PerspectiveCamera::new(75.0, inner_width / inner_height, 0.11, 1000.0);
	camera.position().set_z(z);
	camera
}

fn attach_renderer(window: &Window, renderer: WebGLRenderer, inner_width: f64, inner_height: f64) -> WebGLRenderer {
	renderer.set_size(inner_width as isize, inner_height as isize, false);
	let node = renderer.dom_element().dyn_into::<web_sys::Node>().expect("renderer's element must be node");
	window.body().append_child(&node).expect("body must append renderer's dom element");
	renderer
}
