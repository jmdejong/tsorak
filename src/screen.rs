
use crate::{
	input::Input,
	screenbuffer::ScreenBuffer
};
use std::io;

pub trait Screen {

	fn finalize(self);
	fn update_size(&mut self);
	
	fn write_screen_buffer(&self, buffer: &ScreenBuffer, dest_pos: (usize, usize), src_pos: (usize, usize), size: (usize, usize));
	
	fn await_keyboard_input(&self) -> Option<Input>;
	
	fn width(&self) -> usize;
	fn height(&self) -> usize;
	
}

pub struct DebugScreen(pub usize, pub usize);

impl Screen for DebugScreen {
	fn finalize(self) {}
	fn update_size(&mut self) {}
	
	fn write_screen_buffer(&self, buffer: &ScreenBuffer, (dest_x, dest_y): (usize, usize), (src_x, src_y): (usize, usize), (width, height): (usize, usize)){
		println!("{}", buffer.to_lines().join("|\n"));
	}
	
	fn width(&self) -> usize {
		self.0
	}
	fn height(&self) -> usize {
		self.1
	}
		
	
	fn await_keyboard_input(&self) -> Option<Input> {
	
		let mut stra = String::new();
		io::stdin()
			.read_line(&mut stra)
			.expect("failed to read input.");
		Some(match stra.chars().next().unwrap() {
			'w' => Input::Forward,
			's' => Input::Back,
			'a' => Input::TurnLeft,
			'd' => Input::TurnRight,
			'q' => Input::MoveLeft,
			'e' => Input::MoveRight,
			_ => Input::Nothing
		})
	}
}
