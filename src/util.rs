


use cgmath::{
	Vector2 as Vector2_,
	Vector3 as Vector3_,
	Point2 as Point2_,
	Point3 as Point3_,
	Matrix4 as Matrix4_,
	Rad as Rad_
};


pub type Vector2 = Vector2_<f32>;
pub type Vector3 = Vector3_<f32>;
pub type Point2 = Point2_<f32>;
pub type Point3 = Point3_<f32>;
pub type Matrix4 = Matrix4_<f32>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rad(pub f32);
impl Rad {
	pub fn to_cgmath_rad(&self) -> Rad_<f32>{
		Rad_(self.0)
	}
	pub fn sin(&self) -> f32 {
		self.0.sin()
	}
	pub fn cos(&self) -> f32 {
		self.0.cos()
	}
}


#[macro_export]
macro_rules! hashmap {
	{ $($key:expr => $value:expr ),* } => {{
		#[allow(unused_mut)]
		let mut h = std::collections::HashMap::new();
		$(
			h.insert($key, $value);
		)*
		h
	}}
}
