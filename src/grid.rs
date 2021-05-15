


pub struct Grid<T> {
	data: Vec<T>,
	width: usize,
	height: usize,
}


impl<T: Clone> Grid<T> {
	
	
	
	pub fn create(width: usize, height: usize, val: &T) -> Grid<T> {
		let mut grid = Grid{data: Vec::new(), width, height};
		grid.fill(val);
		grid
	}
	
	
	pub fn from_lines<I: Iterator<Item = T>, J: Iterator<Item = I>>(width: usize, height: usize, lines: J, def: &T) -> Grid<T> {
		let mut grid = Self::create(width, height, def);
		for (y, line) in lines.take(height).enumerate() {
			for (x, val) in line.take(width).enumerate() {
				grid.set(x, y, val.clone());
			}
		}
		grid
	}
	
	pub fn fill_with<F>(&mut self, f: F)
	where
		F: FnMut() -> T,
	{
		self.data.clear();
		self.data.resize_with(self.width * self.height, f);
	}
	
	pub fn fill(&mut self, val: &T) {
		self.fill_with(|| val.clone());
	}
	
	fn index(&self, x: usize, y: usize) -> Option<usize> {
		if x >= self.width || y >= self.height {
			None
		} else {
			Some(x + y * self.width)
		}
	}
	
	pub fn iter_cells(&self) -> GridIterator<T> {
		GridIterator{grid: self, x: 0, y: 0}
	}
	
	pub fn get(&self, x: usize, y: usize) -> Option<&T> {
		self.data.get(self.index(x, y)?)
	}
	
	pub fn set(&mut self, x: usize, y: usize, val: T) -> Option<()> {
		let i = self.index(x, y)?;
		self.data[i] = val;
		Some(())
	}
	
	pub fn width(&self) -> usize {
		self.width
	}
	
	pub fn height(&self) -> usize {
		self.height
	}
}


pub struct GridIterator<'a, T: 'a>{
	grid: &'a Grid<T>,
	x: usize,
	y: usize
}

impl<'a, T: Clone> Iterator for GridIterator<'a, T> {
    type Item = ((usize, usize), &'a T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.x += 1;
        if self.x == self.grid.width {
			self.x = 0;
			self.y += 1;
			if self.y == self.grid.height {
				return None;
			}
		}
		Some(((self.x, self.y), self.grid.get(self.x, self.y).unwrap()))
    }
//     #[inline]
//     fn size_hint(&self) -> (usize, Option<usize>) {
//         self.base.size_hint()
//     }
}
	
