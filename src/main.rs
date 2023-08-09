pub mod three;
pub mod dom;
pub mod basics;

use std::f64::consts::PI;
use wasm_bindgen::prelude::*;
use dom::Window;
use three::{WebGLRenderer};
use crate::dom::{Error};
use crate::three::{PerspectiveCamera, Scene};
use crate::three::wrap::set_animation_loop_with_forget;

fn main() -> Result<(), Error> {
	wasm_logger::init(wasm_logger::Config::default());
	log::info!("Boo!");
	let window = Window::connect();
	let (inner_width, inner_height) = window.inner_size();
	let (scene, camera, cube_rotation) = {
		let scene = Scene::new();
		let camera = three::immersive_camera(inner_width, inner_height);
		scene.add_camera(&camera);
		let mesh = {
			let geometry = three::BoxGeometry::new(2., 1., 1.);
			let material = three::MeshBasicMaterial::new();
			material.color().set(0., 0., 255.);
			three::Mesh::new(geometry, material)
		};
		mesh.position().set_xyz(0., 1.6, -3.);
		let cube_rotation = mesh.rotation();
		cube_rotation.set(0., 0., 0.);
		scene.add(&mesh);
		(scene, camera, cube_rotation)
	};
	let renderer = WebGLRenderer::new();
	renderer.set_pixel_ratio(window.as_dom_window().device_pixel_ratio());
	renderer.set_size(inner_width as isize, inner_height as isize, false);
	renderer.xr().set_enabled(true);
	const FULL_ROTATION: f64 = 2. * PI;
	const RPM: f64 = FULL_ROTATION / 60.;
	const VELOCITY: f64 = 6. * RPM;
	set_animation_loop_with_forget(&renderer, {
		let renderer = renderer.clone().unchecked_into::<WebGLRenderer>();
		let camera = camera.clone().unchecked_into::<PerspectiveCamera>();
		move |time| {
			let seconds = time / 1000.;
			let distance = VELOCITY * seconds;
			let (x, y) = (distance, 3. * distance);
			cube_rotation.set(x, y, 0.0);
			renderer.render(&scene, &camera);
		}
	});
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
