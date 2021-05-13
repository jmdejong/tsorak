


use crate::brush::Brush;


pub struct ScreenBuffer {
	pub width: usize,
	pub height: usize,
	cells: Vec<Brush>
}

impl ScreenBuffer{
	pub fn new(width: usize, height: usize) -> ScreenBuffer {
		let mut cells = Vec::new();
		cells.resize_with((width * height) as usize, Brush::default);
		ScreenBuffer {
			width,
			height,
			cells
		}
	}
	
	pub fn get_cell(&self, (x, y): (usize, usize)) -> Brush {
		self.cells[(x + y * self.width) as usize]
	}
	
	pub fn clear(&mut self) {
		self.cells.clear();
		self.cells.resize_with((self.width * self.height) as usize, Brush::default);
	}
	
	pub fn set(&mut self, (x, y): (usize, usize), brush: Brush){
		let i = (x + y * self.width) as usize;
		self.cells[i] = brush;
	}
	
	pub fn to_lines(&self) -> Vec<String>{
		(0..self.height).map(|y| {
			(0..self.width).map(|x| self.get_cell((x, y)).ch).collect::<String>()
		}).collect()
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
