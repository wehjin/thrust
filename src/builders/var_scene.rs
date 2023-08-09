use std::collections::HashMap;
use wasm_bindgen::JsCast;
use crate::basics::{Rgb, Whd, Xyz};
use crate::builders::{Item, VarMeshBuilder};
use crate::three;

pub struct VarSceneBuilder {
	scene: three::Scene,
	items: HashMap<String, Item>,
	var_mesh_builder: Option<VarMeshBuilder>,
}

impl VarSceneBuilder {
	pub fn new() -> Self {
		VarSceneBuilder {
			scene: three::Scene::new(),
			items: HashMap::new(),
			var_mesh_builder: None,
		}
	}
	pub fn find_item(&self, name: impl AsRef<str>) -> Option<&Item> { self.items.get(name.as_ref()) }
	pub fn to_var_scene(self) -> VarScene {
		let VarSceneBuilder { scene, items, var_mesh_builder } = self;
		assert!(var_mesh_builder.is_none());
		VarScene { scene, items }
	}
}

impl VarSceneBuilder {
	pub fn add_mat_mesh_basic(&mut self, name: impl AsRef<str>, Rgb { r, g, b }: Rgb) {
		let material = three::MeshBasicMaterial::new();
		material.color().set(r, g, b);
		let item = Item::MeshBasicMat(material);
		self.items.insert(name.as_ref().to_string(), item);
	}
	pub fn find_mesh_basic_mat(&self, name: impl AsRef<str>) -> Option<&three::MeshBasicMaterial> {
		self.find_item(name).map(|item| item.as_mesh_basic_geo()).flatten()
	}
}

impl VarSceneBuilder {
	pub fn add_geo_box(&mut self, name: impl AsRef<str>, Whd { w, h, d }: Whd) {
		let item = Item::BoxGeo(three::BoxGeometry::new(w, h, d));
		self.items.insert(name.as_ref().to_string(), item);
	}
	pub fn find_box_geo(&self, name: impl AsRef<str>) -> Option<&three::BoxGeometry> {
		self.find_item(name).map(|item| item.as_box_geo()).flatten()
	}
}

impl VarSceneBuilder {
	pub fn add_var_mesh(&mut self, name: impl AsRef<str>, geometry: impl AsRef<str>, material: impl AsRef<str>) {
		let geometry = self.find_box_geo(geometry).unwrap();
		let material = self.find_mesh_basic_mat(material).unwrap();
		let mesh = three::Mesh::new(geometry, material);
		mesh.position().set_xyz(0., 1.6, -3.);
		let var_mesh_builder = VarMeshBuilder::new(name, mesh);
		self.var_mesh_builder = Some(var_mesh_builder);
	}
	pub fn add_close_var_rot(&mut self, name: impl AsRef<str>, xyz: Xyz) {
		let mut var_mesh_builder = self.var_mesh_builder.take().unwrap();
		var_mesh_builder.add_var_rot(name, xyz);
		self.var_mesh_builder = Some(var_mesh_builder);
	}
	pub fn close_var_mesh(&mut self) {
		let var_mesh = self.var_mesh_builder.take().unwrap();
		let (name, var_mesh) = var_mesh.to_var_mesh();
		if let Some((rot_name, var_rot)) = var_mesh.find_var_rot() {
			let item = Item::VarRot(var_rot.clone());
			self.items.insert(rot_name.to_string(), item);
		}
		self.scene.add(var_mesh.as_three_mesh());
		self.items.insert(name, Item::VarMesh(var_mesh));
	}
}

pub struct VarScene {
	scene: three::Scene,
	items: HashMap<String, Item>,
}

impl VarScene {
	pub fn as_three_scene(&self) -> &three::Scene { &self.scene }
	pub fn to_three_scene(&self) -> three::Scene { self.scene.clone().unchecked_into() }
	pub fn update_rot_var(&self, rot_name: impl AsRef<str>, xyz: Xyz) {
		let item = self.items.get(rot_name.as_ref()).unwrap();
		let var_rot = item.as_var_rot().unwrap();
		var_rot.set_val(xyz)
	}
}
