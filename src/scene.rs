

use crate::{
	util::{Vector2, Point2, Vector3, Point3, Matrix2},
	brush::Brush,
	texture::Texture
};
use cgmath::prelude::*;


#[derive(Debug, Clone)]
pub struct Plane {
	height: f32,
	texture: Texture
}

impl Plane {
	
	pub fn intersect_ray(&self, origin: Point3, mut direction: Vector3) -> Option<Hit> {
		if direction.z == 0.0 {
			return None
		}
		let t = -(origin.z - self.height) / direction.z;
		if t <= 0.0 {
			return None
		}
		let u = origin.x + t * direction.x;
		let v = origin.y + t * direction.y;
		Some(Hit {distance: t , brush: self.texture.get(u, v)?})
	}
}


#[derive(Debug, Clone)]
pub enum Shape {
	Wall(Point3, Point3),
// 	Plane(f32),
	Sprite(Point3, (f32, f32))
}


#[derive(Debug, Clone)]
pub struct ShapeObject {
	texture: Texture,
	shape: Shape
}


impl ShapeObject {

	pub fn new(shape: Shape, texture: Texture) -> Self {
		Self {shape, texture }
	}
	
	pub fn intersect_ray2d(&self, origin: Point2, mut direction: Vector2) -> Option<Column> {
		let mat = Matrix2::look_at_stable(direction, false);
		match self.shape {
			Shape::Wall(p0, p1) => {
				let dir = p1 - p0;
				let d = direction.x * dir.y - direction.y * dir.x;
				if d == 0.0 {
					return None;
				}
				let t_part = (origin.x - p0.x) * (-dir.y) - (origin.y - p0.y) * (-dir.x);
				let u_part = direction.x * (origin.y - p0.y) - direction.y * (origin.x - p0.x);
				let t = t_part / d;
				let u = u_part / d;
				if t <= 0.0 || u < 0.0 || u > 1.0 {
					None
				} else {
					Some(Column{tex: &self.texture, t, u, bottom: p0.z, top: p1.z})
				}
			}
// 				let p0 = mat * (Point2::new(x0, y0) - origin);
// 				let p1 = mat * (Point2::new(x1, y1) - origin);
// 				let dy = p1.y - p0.y;
// 				if dy == 0.0 {
// 					return None;
// 				}
// 				let u = -p0.y / dy;
// 				let t = p0.x * (1.0 - u) + u * p1.x;
// 				if t <= 0.0 || u < 0.0 || u > 1.0 {
// 					None
// 				} else {
// 					Some(Column{tex: &self.texture, t, u, bottom: z0, top: z1})
// 				}
// 			}
			Shape::Sprite(pos, (width, height)) => {
				None
			}
		}
	}
	
// 	pub fn intersect_ray(&self, origin: Point3, mut direction: Vector3) -> Option<Hit> {
// 		direction = direction.normalize();
// 		let ray_end = origin + direction;
// 		match self.shape {
// 			Shape::Wall(p0, p1) => {
// 				let dir = p1 - p0;
// 				let d = direction.x * dir.y - direction.y * dir.x;
// 				if d == 0.0 {
// 					return None;
// 				}
// 				let t_part = (origin.x - p0.x) * (-dir.y) - (origin.y - p0.y) * (-dir.x);
// 				let u_part = direction.x * (origin.y - p0.y) - direction.y * (origin.x - p0.x);
// 				let t = t_part / d;
// 				let u = u_part / d;
// 				let hit_z = origin.z + t * direction.z;
// 				let v = (hit_z - p0.z) / dir.z;
// 				if t <= 0.0 || u < 0.0 || u > 1.0 || v < 0.0 || v > 1.0 {
// 					return None;
// 				}
// 				Some(Hit { distance: t, brush: self.texture.get(u, v)?})
// 			}
// 			Shape::Sprite(pos, (width, height)) => {
// 				None
// 			}
// // 				let direction2 = direction.truncate().normalize();
// // 				let diff = pos - origin;
// // 				let d_u = direction2.x * diff.y + direction2.y * diff.x;
// // 				let u = (d_u + 0.5) / width;
// // 				if u < 0.0 || y > 1.5 {
// // 					return None;
// // 				}
// // 			}
// 		}
// 	}
	
	pub fn moved(&self, d: Vector3) -> Self {
		let shape = match self.shape {
// 			Shape::Plane(height) => Shape::Plane(height),
			Shape::Wall(p0, p1) => Shape::Wall(p0 + d, p1 + d),
			Shape::Sprite(pos, size) => Shape::Sprite(pos + d, size)
		};
		Self { shape, texture: self.texture.clone()}
	}
}

#[derive(Debug, Clone)]
pub struct Hit {
	pub distance: f32,
	pub brush: Brush
}


#[derive(Debug, Clone)]
pub struct Column<'a> {
	pub tex: &'a Texture,
	pub t: f32,
	pub u: f32,
	pub top: f32,
	pub bottom: f32
	
}

impl<'a> Column<'a> {
	pub fn get_hit(&self, origin_z: f32, dz: f32) -> Option<Brush> {
		
		let hit_z = origin_z + dz * self.t;
		let v = (hit_z - self.bottom) / (self.top - self.bottom);
		if v >= 0.0 && v <= 1.0 {
			self.tex.get(self.u, v)
		} else {
			None
		}
	}
}

#[derive(Debug, Clone)]
pub struct Scene {
	pub shapes: Vec<ShapeObject>,
	pub planes: Vec<Plane>
}


impl Scene {
	pub fn new(planes: &[Plane], shapes: &[ShapeObject]) -> Scene {
		Self {
			planes: planes.into_iter().cloned().collect(),
			shapes: shapes.into_iter().cloned().collect()
		}
	}
	
	pub fn plane_intersections(&self, origin: Point3, direction: Vector3) -> Option<Hit> {
		// todo: sort planes by height and start from origin z
		self.planes.iter()
			.filter_map(|plane| plane.intersect_ray(origin, direction))
			.min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
	}
	
	pub fn shapes_on_ray2d(&self, origin: Point2, direction: Vector2) -> Vec<Column> {
		// todo: spatial partitioning; bounding rects
		let mut columns: Vec<Column> = self.shapes.iter()
			.filter_map(|shape| shape.intersect_ray2d(origin, direction))
			.filter(|column| column.top > column.bottom)
			.collect();
		columns.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
		columns
	}
}


pub fn wall(p0: (f32, f32, f32), p1: (f32, f32, f32), texture: Texture) -> ShapeObject {
	ShapeObject::new(Shape::Wall(Point3::from(p0), Point3::from(p1)), texture)
}

pub fn plane(height: f32, texture: Texture) -> Plane {
	Plane{height, texture}
}
