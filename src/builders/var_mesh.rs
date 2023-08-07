use std::ops::Deref;
use std::rc::{Rc, Weak};
use crate::basics::Xyz;

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

