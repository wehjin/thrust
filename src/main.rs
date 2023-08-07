pub mod three;
pub mod dom;
pub mod basics;
pub mod builders;

use std::f64::consts::PI;
use wasm_bindgen::prelude::*;
use dom::Window;
use three::{WebGLRenderer};
use crate::builders::{VarSceneBuilder};
use crate::dom::{Error};
use crate::three::PerspectiveCamera;
use crate::three::wrap::set_animation_loop_with_forget;

fn main() -> Result<(), Error> {
	wasm_logger::init(wasm_logger::Config::default());
	log::info!("Boo!");

	let (x, y) = (0.0, 0.0);
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
		const FULL_ROTATION: f64 = 2. * PI;
		const VELOCITY: f64 = 6. * FULL_ROTATION / 60.;
		set_animation_loop_with_forget(&renderer, move |time| {
			let seconds = time / 1000.;
			let distance = VELOCITY * seconds;
			let (x, y) = (distance, 3. * distance);
			var_scene.update_rot_var(CUBE_ROTATION, (x, y, 0.0).into());
			frame_renderer.render(var_scene.as_three_scene(), &frame_camera);
		});
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
