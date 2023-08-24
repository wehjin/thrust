use wasm_bindgen::prelude::*;
use crate::three::{Object3D, Vector3};

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
	#[wasm_bindgen(extends = Object3D)]
	#[derive(Clone, Default)]
	pub type PerspectiveCamera;
	#[wasm_bindgen(constructor)]
	pub fn new(fov: f64, aspect: f64, near: f64, far: f64) -> PerspectiveCamera;
	#[wasm_bindgen(method, getter)]
	pub fn position(this: &PerspectiveCamera) -> Vector3;
	#[wasm_bindgen(method, setter)]
	pub fn set_aspect(this: &PerspectiveCamera, val: f64);
	#[wasm_bindgen(method, js_name = updateProjectionMatrix)]
	pub fn update_projection_matrix(this: &PerspectiveCamera);
}


pub fn immersive_camera(inner_width: f64, inner_height: f64) -> PerspectiveCamera {
	// As suggested in the example at https://immersiveweb.dev/#three.js
	let camera = PerspectiveCamera::new(50., inner_width / inner_height, 0.1, 100.);
	camera.position().set_xyz(0., 1.6, 3.);
	camera
}