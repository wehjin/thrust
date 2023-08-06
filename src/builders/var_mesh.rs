use std::ops::Deref;
use std::rc::{Rc, Weak};
use crate::basics::Xyz;
use crate::builders::{Item, VarSceneBuilder};
use crate::three;

#[derive(Clone)]
pub struct VarRot {
	mesh: Weak<three::Mesh>,
}

impl VarRot {
	pub fn set_val(&self, Xyz { x, y, z }: Xyz) {
		let mesh = self.mesh.upgrade().unwrap();
		mesh.rotation().set(x, y, z);
	}
}

pub enum EndVarMeshOption {
	KeepVarRot(String)
}

pub struct VarMesh {
	mesh: Rc<three::Mesh>,
	var_rot: Option<(String, VarRot)>,
}

impl VarMesh {
	pub fn as_three_mesh(&self) -> &three::Mesh { self.mesh.deref() }
	pub fn find_var_rot(&self) -> &Option<(String, VarRot)> { &self.var_rot }
}

pub struct VarMeshBuilder {
	name: String,
	mesh: three::Mesh,
	var_rot: Option<(String, Xyz)>,
}

impl VarMeshBuilder {
	pub fn new(name: impl AsRef<str>, mesh: three::Mesh) -> Self {
		VarMeshBuilder {
			name: name.as_ref().to_string(),
			mesh,
			var_rot: None,
		}
	}
	pub fn add_var_rot(&mut self, name: impl AsRef<str>, xyz: Xyz) {
		self.var_rot = Some((name.as_ref().to_string(), xyz));
	}
	pub fn to_var_mesh(self) -> (String, VarMesh) {
		let VarMeshBuilder { name, mesh, var_rot } = self;
		let strong = Rc::new(mesh);
		let weak = Rc::downgrade(&strong);
		let var_mesh = VarMesh {
			mesh: strong,
			var_rot: var_rot.map(|(name, _)| (name, VarRot { mesh: weak })),
		};
		(name, var_mesh)
	}
}

impl VarSceneBuilder {
	pub(crate) fn add_var_mesh(&mut self, name: impl AsRef<str>, geometry: impl AsRef<str>, material: impl AsRef<str>) {
		let geometry = self.find_box_geo(geometry).unwrap();
		let material = self.find_mesh_basic_mat(material).unwrap();
		let mesh = three::Mesh::new(geometry, material);
		let var_mesh_builder = VarMeshBuilder::new(name, mesh);
		self.var_mesh_builder = Some(var_mesh_builder);
	}
	pub(crate) fn add_close_var_rot(&mut self, name: impl AsRef<str>, xyz: Xyz) {
		let mut var_mesh_builder = self.var_mesh_builder.take().unwrap();
		var_mesh_builder.add_var_rot(name, xyz);
		self.var_mesh_builder = Some(var_mesh_builder);
	}
	pub(crate) fn close_var_mesh(&mut self) {
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