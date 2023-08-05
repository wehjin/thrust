mod three;
mod dom;

use wasm_bindgen::prelude::*;


//noinspection RsUnresolvedReference
fn main() -> Result<(), dom::Error> {
	wasm_logger::init(wasm_logger::Config::default());
	log::info!("Boo!");
	let window = dom::window();
	let inner_width: JsValue = window.inner_width()?;
	let inner_height: JsValue = window.inner_height()?;
	let inner_width = inner_width.as_f64().unwrap();
	let inner_height = inner_height.as_f64().unwrap();
	log::info!("width: {}, height: {}", inner_width, inner_height);
	let body = dom::body();
	let scene = three::Scene::new();
	let camera = three::PerspectiveCamera::new(75.0, inner_width / inner_height, 0.11, 1000.0);
	let renderer = three::WebGLRenderer::new();
	renderer.set_size(inner_width as isize, inner_height as isize, false);
	let dom_element = renderer.dom_element();
	let dom_element = dom_element.dyn_into::<web_sys::Node>()?;
	body.append_child(&dom_element)?;
	let geometry = three::BoxGeometry::new(1.0, 1.0, 1.0);
	let material = three::MeshBasicMaterial::new();
	material.color().set(0.0, 255.0, 0.0);
	let cube = three::Mesh::new(geometry, material);
	scene.add(&cube);
	camera.position().set_z(5.0);

	let (mut x, mut y) = (0.0, 0.0);
	dom::animate_frames(move || {
		(x, y) = (x + 0.01, y + 0.02);
		cube.rotation().set(x, y, 0.0);
		renderer.render(&scene, &camera);
		true
	});
	Ok(())
}
