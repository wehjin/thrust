use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
	pub type Vector3;
	#[wasm_bindgen(constructor)]
	pub fn new(x: f64, y: f64, z: f64) -> Vector3;
	#[wasm_bindgen(method, setter)]
	pub fn set_z(this: &Vector3, val: f64);

	pub type Euler;
	#[wasm_bindgen(method, getter)]
	pub fn x(this: &Euler) -> JsValue;
	#[wasm_bindgen(method, getter)]
	pub fn y(this: &Euler) -> JsValue;
	#[wasm_bindgen(method)]
	pub fn set(this: &Euler, x: f64, y: f64, z: f64);

	pub type Color;
	#[wasm_bindgen(constructor)]
	pub fn new(r: f64, g: f64, b: f64) -> Color;
	#[wasm_bindgen(method)]
	pub fn set(this: &Color, r: f64, g: f64, b: f64);

	pub type BoxGeometry;
	#[wasm_bindgen(constructor)]
	pub fn new(width: f64, height: f64, depth: f64) -> BoxGeometry;

	pub type MeshNormalMaterial;
	#[wasm_bindgen(constructor)]
	pub fn new() -> MeshNormalMaterial;

	pub type MeshBasicMaterial;
	#[wasm_bindgen(constructor)]
	pub fn new() -> MeshBasicMaterial;
	#[wasm_bindgen(method, getter)]
	pub fn color(this: &MeshBasicMaterial) -> Color;

	pub type Mesh;
	#[wasm_bindgen(constructor)]
	pub fn new(geometry: &BoxGeometry, material: &MeshBasicMaterial) -> Mesh;
	#[wasm_bindgen(method, getter)]
	pub fn rotation(this: &Mesh) -> Euler;

	pub type Scene;
	#[wasm_bindgen(constructor)]
	pub fn new() -> Scene;
	#[wasm_bindgen(method)]
	pub fn add(this: &Scene, mesh: &Mesh);

	pub type PerspectiveCamera;
	#[wasm_bindgen(constructor)]
	pub fn new(fov: f64, aspect: f64, near: f64, far: f64) -> PerspectiveCamera;
	#[wasm_bindgen(method, getter)]
	pub fn position(this: &PerspectiveCamera) -> Vector3;

	pub type WebGLRenderer;
	#[wasm_bindgen(constructor)]
	pub fn new() -> WebGLRenderer;
	#[wasm_bindgen(method, js_name = "setSize")]
	pub fn set_size(this: &WebGLRenderer, width: isize, height: isize, update_style: bool);
	#[wasm_bindgen(method, getter, js_name = "domElement")]
	pub fn dom_element(this: &WebGLRenderer) -> JsValue;
	#[wasm_bindgen(method)]
	pub fn render(this: &WebGLRenderer, scene: &Scene, camera: &PerspectiveCamera);
}
