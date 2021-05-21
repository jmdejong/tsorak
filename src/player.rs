
use crate::{
	util::{Point3, Vector3, Matrix4, Rad},
	input::Input,
	gamefield::GameField
};

pub struct Player {
	pub pos: Point3,
	pub dir: i32
}

const speed: f32 = 0.5;

impl Player {
	
	pub fn view_angle(&self) -> Rad {
		Rad(self.dir as f32 / 12.0 * std::f32::consts::PI)
	}
	
	pub fn view_direction(&self) -> Vector3 {
		let a = self.view_angle();
		Vector3::new(a.0.sin(), a.0.cos(), 0.0)
	}
	
	pub fn domove(&mut self, input: Input, field: &GameField){
		let (mx, my, mz) = match input {
			Input::Forward => (1.0, 0.0, 0.0),
			Input::MoveLeft => (0.0, -1.0, 0.0),
			Input::Back => (-1.0, 0.0, 0.0),
			Input::MoveRight => (0.0, 1.0, 0.0),
			_ => (0.0, 0.0, 0.0)
		};
		let movement : Vector3 = Vector3::new(mx, my, mz);
		let p = self.pos + (Matrix4::from_angle_z(self.view_angle().to_cgmath_rad()) * movement.extend(1.0)).truncate() * speed; 
		if field.is_accessible(p.x.floor() as usize, p.y.floor() as usize) {
			self.pos = p;
		} else if field.is_accessible(p.x.floor() as usize, self.pos.y.floor() as usize) {
			self.pos.x = p.x;
		} else if field.is_accessible(self.pos.x.floor() as usize, p.y.floor() as usize) {
			self.pos.y = p.y
		}
		self.dir +=  match input {
			Input::TurnLeft => -1,
			Input::TurnRight => 1,
			_ => 0
		};
	}
	
}
