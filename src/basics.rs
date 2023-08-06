pub struct Whd {
	pub w: f64,
	pub h: f64,
	pub d: f64,
}

impl From<(f64, f64, f64)> for Whd {
	fn from((w, h, d): (f64, f64, f64)) -> Self { Whd { w, h, d } }
}

pub struct Rgb {
	pub r: f64,
	pub g: f64,
	pub b: f64,
}

impl From<(f64, f64, f64)> for Rgb {
	fn from((r, g, b): (f64, f64, f64)) -> Self { Rgb { r, g, b } }
}

pub struct Xyz {
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

impl From<(f64, f64, f64)> for Xyz {
	fn from((x, y, z): (f64, f64, f64)) -> Self { Xyz { x, y, z } }
}
