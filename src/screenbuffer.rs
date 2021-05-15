

use std::collections::HashMap;
use crate::brush::Brush;


#[derive(Debug, Clone)]
pub struct ScreenBuffer {
	width: usize,
	height: usize,
	cells: Vec<Option<Brush>>
}

impl ScreenBuffer{
	pub fn new(width: usize, height: usize) -> ScreenBuffer {
		let mut cells = Vec::new();
		cells.resize((width * height) as usize, None);
		ScreenBuffer {
			width,
			height,
			cells
		}
	}
	
	pub fn get(&self, (x, y): (usize, usize)) -> Option<Brush> {
		self.cells[(x + y * self.width) as usize]
	}
	
	pub fn getf(&self, (x, y): (f32, f32)) -> Option<Brush> {
		self.get((
			(x.max(0.0) as usize).min(self.width - 1),
			(y.max(0.0) as usize).min(self.height - 1)
		))
	}
	
	pub fn fill(&mut self, brush: Option<Brush>){
		self.cells.clear();
		self.cells.resize((self.width * self.height) as usize, brush);
	}
	
	pub fn set(&mut self, (x, y): (usize, usize), brush: Option<Brush>){
		let i = (x + y * self.width) as usize;
		self.cells[i] = brush;
	}
	
	pub fn to_lines(&self) -> Vec<String>{
		(0..self.height).map(|y| {
			(0..self.width).map(|x| self.get((x, y)).map(|cell| cell.ch).unwrap_or(' ')).collect::<String>()
		}).collect()
	}
	
	pub fn from_lines(width: usize, height: usize, lines: &[&str], mapping: &HashMap<char, Brush>) -> ScreenBuffer {
		let mut buffer = ScreenBuffer::new(width, height);
		for (y, line) in lines.iter().take(height).enumerate() {
			for (x, ch) in line.chars().take(width).enumerate() {
				let brush : Option<Brush> = mapping.get(&ch).cloned();
				buffer.set((x, y), brush);
			}
		}
		buffer
	}
	
	pub fn width(&self) -> usize {
		self.width
	}
	pub fn height(&self) -> usize {
		self.height
	}
}




#[cfg(test)]
mod tests {

	use super::*;
	use crate::brush::{brush, Color};
	
	
	const style: Style = Style{fg: Color::LightGray, bg: Color::Black};
	
	
	#[test]
	fn screenbuffer_sets() {
		let mut buffer = ScreenBuffer::new(10, 10);
		buffer.set((5, 7), brush('@', style));
		let c = buffer.get_cell((5,7));
		assert_eq!(c.ch, '@');
		assert_eq!(c.style, style);
		buffer.set((5, 8), brush('0', style));
		buffer.set((5, 7), brush('1', style));
		buffer.set((5, 7), brush('2', style));
		let c = buffer.get_cell((5,7));
		assert_eq!(c.ch, '2');
		assert_eq!(c.style, style);
		buffer.set((5, 7), brush('#', style));
		let c = buffer.get_cell((5,7));
		assert_eq!(c.ch, '#');
		assert_eq!(c.style, style);
		buffer.clear();
		let c = buffer.get_cell((5,7));
		assert_eq!(c.ch, ' ');
	}
	
	#[test]
	fn to_lines(){
		let mut buffer = ScreenBuffer::new(5, 5);
		buffer.set((1, 1), brush('b', style));
		buffer.set((3, 1), brush('f', style));
		buffer.set((4, 4), brush('z', style));
		buffer.set((0, 0), brush('a', style));
		buffer.set((1, 1), brush('h', style));
		buffer.set((1, 1), brush('g', style));
		assert_eq!(
			buffer.to_lines(),
			vec!(
				"a    ",
				" g f ",
				"     ",
				"     ",
				"    z"
			)
		);
		buffer.clear();
		assert_eq!(
			buffer.to_lines(),
			vec!(
				"     ",
				"     ",
				"     ",
				"     ",
				"     "
			)
		);
	}
}
