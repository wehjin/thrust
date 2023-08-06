pub mod three;
pub mod dom;

use wasm_bindgen::prelude::*;
use dom::Window;
use three::{BoxGeometry, Mesh, MeshBasicMaterial, PerspectiveCamera, Scene, WebGLRenderer};
use crate::dom::{Error, FrameNext};

fn main() -> Result<(), Error> {
	wasm_logger::init(wasm_logger::Config::default());
	log::info!("Boo!");

	let (mut x, mut y) = (0.0, 0.0);
	let cube = create_cube(x, y);
	let scene = populate_scene(Scene::new(), &cube);
	let window = Window::connect();
	let (camera, renderer) = {
		let (inner_width, inner_height) = window.inner_size();
		let camera = create_camera(5.0, inner_width, inner_height);
		let renderer = attach_renderer(&window, WebGLRenderer::new(), inner_width, inner_height);
		(camera, renderer)
	};
	window.animate_frames(move || {
		(x, y) = (x + 0.01, y + 0.02);
		cube.rotation().set(x, y, 0.0);
		renderer.render(&scene, &camera);
		FrameNext::Repeat
	});
	Ok(())
}

fn populate_scene(scene: Scene, cube: &Mesh) -> Scene {
	scene.add(&cube);
	scene
}

fn create_cube(rot_x: f64, rot_y: f64) -> Mesh {
	let geometry = BoxGeometry::new(1.0, 1.0, 1.0);
	let material = create_material();
	let cube = Mesh::new(geometry, material);
	cube.rotation().set(rot_x, rot_y, 0.0);
	cube
}

fn create_material() -> MeshBasicMaterial {
	let material = MeshBasicMaterial::new();
	material.color().set(0.0, 255.0, 0.0);
	material
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
