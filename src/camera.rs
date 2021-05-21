
use cgmath::Matrix2;
use crate::util::{Point3, Rad, Vector2};

#[derive(Debug, Clone)]
pub struct Camera {
	hor_side: f32,
	vert_side: f32,
	pub position: Point3,
	pub dir: Rad
}

impl Camera {
	
	pub fn new(hor_side: f32, vert_side: f32) -> Self {
		Self {hor_side, vert_side, position: Point3::new(0.0, 0.0, 0.0), dir: Rad(0.0)}
	}

	pub fn calculate_hor_rays(&self, screen_width: usize) -> Vec<Vector2> {
		let m = self.rotation();
		(0..screen_width)
			.map(|col| {
				let x = (col as f32 / screen_width as f32 * 2.0 - 1.0) * self.hor_side;
				m * Vector2::new(1.0, x)
			})
			.collect()
	}
	
	
	pub fn calculate_vert_angles(&self, screen_height: usize) -> Vec<f32> {
		(0..screen_height)
			.map(|row| {
				(row as f32 / screen_height as f32 * 2.0 - 1.0) * self.vert_side
			})
			.collect()
	}
	
	pub fn move_view(&mut self, position: Point3, dir: Rad){
		self.position = position;
		self.dir = dir;
	}
	
	pub fn rotation(&self) -> Matrix2<f32> {
		Matrix2::from_angle(self.dir.to_cgmath_rad())
	}
	
}
