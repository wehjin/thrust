use std::ops::Deref;
use std::rc::{Rc, Weak};
use crate::basics::Xyz;
use crate::builders::{Item, VarSceneBuilder};
use crate::three;

pub struct VarRot {
	mesh: Weak<three::Mesh>,
}

impl VarRot {
	pub fn set_val(&self, Xyz { x, y, z }: Xyz) {
		let mesh = self.mesh.upgrade().unwrap();
		mesh.rotation().set(x, y, z);
	}
}

pub struct VarMesh {
	mesh: Rc<three::Mesh>,
	var_rot: Option<VarRot>,
}

impl VarMesh {
	pub fn as_three_mesh(&self) -> &three::Mesh { self.mesh.deref() }
	pub fn find_var_rot(&self) -> &Option<VarRot> { &self.var_rot }
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
		(
			name,
			VarMesh { mesh: strong, var_rot: var_rot.map(|_| VarRot { mesh: weak }) }
		)
	}
}

impl VarSceneBuilder {
	pub(crate) fn start_var_mesh(&mut self, name: impl AsRef<str>, geometry: impl AsRef<str>, material: impl AsRef<str>) {
		let geometry = self.find_box_geo(geometry).unwrap();
		let material = self.find_mesh_basic_mat(material).unwrap();
		let mesh = three::Mesh::new(geometry, material);
		let var_mesh_builder = VarMeshBuilder::new(name, mesh);
		self.var_mesh_builder = Some(var_mesh_builder);
	}
	pub(crate) fn add_var_mesh_rot(&mut self, name: impl AsRef<str>, xyz: Xyz) {
		let mut var_mesh_builder = self.var_mesh_builder.take().unwrap();
		var_mesh_builder.add_var_rot(name, xyz);
		self.var_mesh_builder = Some(var_mesh_builder);
	}
	pub(crate) fn end_var_mesh(&mut self) {
		let var_mesh = self.var_mesh_builder.take().unwrap();
		let (name, var_mesh) = var_mesh.to_var_mesh();
		self.scene.add(var_mesh.as_three_mesh());
		self.items.insert(name, Item::VarMesh(var_mesh));
	}
}