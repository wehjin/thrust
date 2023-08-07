pub mod three;
pub mod dom;
pub mod basics;
pub mod builders;

use wasm_bindgen::prelude::*;
use dom::Window;
use three::{WebGLRenderer};
use crate::builders::{VarSceneBuilder};
use crate::dom::{Error};
use crate::three::PerspectiveCamera;

fn main() -> Result<(), Error> {
	wasm_logger::init(wasm_logger::Config::default());
	log::info!("Boo!");

	let (mut x, mut y) = (0.0, 0.0);
	const GEOMETRY: &'static str = "geometry";
	const MATERIAL: &'static str = "material";
	const CUBE: &'static str = "cube";
	const CUBE_ROTATION: &'static str = "rotation";
	let var_scene = {
		let mut builder = VarSceneBuilder::new();
		builder.add_geo_box(GEOMETRY, (2.0, 1.0, 1.0).into());
		builder.add_mat_mesh_basic(MATERIAL, (255.0, 0.0, 0.0).into());
		builder.add_var_mesh(CUBE, GEOMETRY, MATERIAL);
		builder.add_close_var_rot(CUBE_ROTATION, (x, y, 0.0).into());
		builder.close_var_mesh();
		builder.to_var_scene()
	};
	let window = Window::connect();
	let (inner_width, inner_height) = window.inner_size();

	let camera = three::immersive_camera(inner_width, inner_height);
	var_scene.as_three_scene().add_camera(&camera);

	let renderer = WebGLRenderer::new();
	renderer.set_pixel_ratio(window.as_dom_window().device_pixel_ratio());
	renderer.set_size(inner_width as isize, inner_height as isize, false);
	renderer.xr().set_enabled(true);
	{
		let frame_renderer = renderer.clone().unchecked_into::<WebGLRenderer>();
		let frame_camera = camera.clone().unchecked_into::<PerspectiveCamera>();
		let closure = Closure::new(move |time| {
			let time = time / 1000.;
			let time = time as u32;
			log::info!("Time {}", time);
			let increment = 0.01;
			(x, y) = (x + increment, y + 3.0 * increment);
			var_scene.update_rot_var(CUBE_ROTATION, (x, y, 0.0).into());
			frame_renderer.render(var_scene.as_three_scene(), &frame_camera);
		});
		renderer.set_animation_loop(&closure);
		// Forget the closure to prevent it from invalidating the callback when
		// this function returns. This is a memory leak so should be done rarely.
		closure.forget();
	}

	window.body().append_child(&renderer.dom_element()).expect("body must append renderer's dom element");
	window.body().append_child(&three::create_button(&renderer)).expect("append button");
	{
		let resize_camera = camera.clone().unchecked_into::<PerspectiveCamera>();
		let resize_renderer = renderer.clone().unchecked_into::<WebGLRenderer>();
		window.add_resize_listener_with_forget(move |_event| {
			let window = Window::connect();
			let (width, height) = window.inner_size();
			resize_camera.set_aspect(width / height);
			resize_camera.update_projection_matrix();
			resize_renderer.set_size(width as isize, height as isize, false);
		});
	}
	Ok(())
}
