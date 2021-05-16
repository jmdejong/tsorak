

use crate::{
	util::{Vector2, Point2, Vector3, Point3},
	brush::Brush,
	texture::Texture
};
use cgmath::prelude::*;

#[derive(Debug, Clone)]
pub enum Shape {
	Wall(Point3, Point3),
	Plane(f32)
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
	
	pub fn intersect_ray(&self, origin: Point3, mut direction: Vector3) -> Option<Hit> {
		direction = direction.normalize();
		let ray_end = origin + direction;
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
				let hit_z = origin.z + t * direction.z;
				let v = (hit_z - p0.z) / dir.z;
				if t <= 0.0 || u < 0.0 || u > 1.0 || v < 0.0 || v > 1.0 {
					return None;
				}
				Some(Hit { distance: t, brush: self.texture.get(u, v)?})
			}
			Shape::Plane(height) => {
				if direction.z == 0.0 {
					return None
				}
				let t = -(origin.z - height) / direction.z;
				if t <= 0.0 {
					return None
				}
				let u = origin.x + t * direction.x;
				let v = origin.y + t * direction.y;
				Some(Hit {distance: t, brush: self.texture.get(u, v)?})
			}
		}
	}
	
	pub fn moved(&self, d: Vector3) -> Self {
		let shape = match self.shape {
			Shape::Plane(height) => Shape::Plane(height),
			Shape::Wall(p0, p1) => Shape::Wall(p0 + d, p1 + d)
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
pub struct Scene {
	pub shapes: Vec<ShapeObject>
}


impl Scene {
	pub fn new(shapes: &[ShapeObject]) -> Scene {
		Self {shapes: 
			shapes.into_iter().cloned().collect()
		}
	}
	
	pub fn shapes_on_ray2d(&self, origin: Point2, direction: Vector2) -> Vec<ShapeObject> {
		// todo: spatial partitioning; bounding rects
		self.shapes.clone()
	}
}


pub fn wall(p0: (f32, f32, f32), p1: (f32, f32, f32), texture: Texture) -> ShapeObject {
	ShapeObject::new(Shape::Wall(Point3::from(p0), Point3::from(p1)), texture)
}

pub fn plane(height: f32, texture: Texture) -> ShapeObject {
	ShapeObject::new(Shape::Plane(height), texture)
}
