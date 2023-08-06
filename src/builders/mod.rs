mod items;
mod var_mesh;


use std::collections::HashMap;
use crate::basics::{Rgb, Whd, Xyz};
use crate::three;
pub use items::*;
pub use var_mesh::*;

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
	pub fn add_val_mesh_basic_mat(&mut self, name: impl AsRef<str>, Rgb { r, g, b }: Rgb) {
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
	pub fn add_val_box_geo(&mut self, name: impl AsRef<str>, Whd { w, h, d }: Whd) {
		let item = Item::BoxGeo(three::BoxGeometry::new(w, h, d));
		self.items.insert(name.as_ref().to_string(), item);
	}
	pub fn find_box_geo(&self, name: impl AsRef<str>) -> Option<&three::BoxGeometry> {
		self.find_item(name).map(|item| item.as_box_geo()).flatten()
	}
}

pub struct VarScene {
	scene: three::Scene,
	items: HashMap<String, Item>,
}

impl VarScene {
	pub fn as_three_scene(&self) -> &three::Scene { &self.scene }
	pub fn set_mesh_rot_val(&self, mesh_name: impl AsRef<str>, _rot_name: impl AsRef<str>, xyz: Xyz) {
		let item = self.items.get(mesh_name.as_ref()).unwrap();
		let var_mesh = item.as_var_mesh().unwrap();
		if let Some(var_rot) = var_mesh.find_var_rot() {
			var_rot.set_val(xyz)
		}
	}
}
