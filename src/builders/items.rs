use crate::builders::{VarMesh, VarRot};
use crate::three;

pub enum Item {
	BoxGeo(three::BoxGeometry),
	MeshBasicMat(three::MeshBasicMaterial),
	VarMesh(VarMesh),
	VarRot(VarRot),
}

impl Item {
	pub fn as_box_geo(&self) -> Option<&three::BoxGeometry> {
		match self {
			Item::BoxGeo(geo) => Some(geo),
			_ => None,
		}
	}
	pub fn as_mesh_basic_geo(&self) -> Option<&three::MeshBasicMaterial> {
		match self {
			Item::MeshBasicMat(mat) => Some(mat),
			_ => None,
		}
	}
}

impl Item {
	pub fn as_var_mesh(&self) -> Option<&VarMesh> {
		match self {
			Item::VarMesh(var_mesh) => Some(&var_mesh),
			_ => None,
		}
	}
	pub fn as_mesh(&self) -> Option<&three::Mesh> {
		self.as_var_mesh().map(|it| it.as_three_mesh())
	}
}

impl Item {
	pub fn as_var_rot(&self) -> Option<&VarRot> {
		match self {
			Item::VarRot(var) => Some(&var),
			_ => None,
		}
	}
}
