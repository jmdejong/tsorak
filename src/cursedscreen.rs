

use pancurses::{
	Window,
	Input
};
use pancurses as curses;

use crate::{
	screenbuffer::ScreenBuffer,
	brush::{Brush, Style},
	input::Input::*,
	input::Input as GameInput,
	screen::Screen
};

#[derive(Debug, Clone)]
struct Colors {
	ncolors: i16,
	npairs: i16
}

impl Colors {
	fn initialize() -> Self {
		curses::start_color();
		let ncolors: i16 = std::cmp::min(curses::COLORS(), 16) as i16;
		let npairs: i16 = ncolors * ncolors;
		for i in 0..npairs {
			curses::init_pair(i, i % ncolors, i / ncolors);
		}
		Self {ncolors, npairs}
	}
	
	fn get(&self, brush: &Brush) -> u32 {
		
		let (fg, bg) = if self.ncolors == 16 {
			(brush.style.fg.0 as u32, brush.style.bg.0 as u32)
		} else if self.ncolors == 8 {
			(brush.backupstyle.fg.0 as u32, brush.backupstyle.bg.0 as u32)
		} else {
			(0, 0)
		};
		curses::COLOR_PAIR(fg + bg * self.ncolors as u32)
	}
}




#[derive(Debug)]
pub struct CursedScreen {
	screen: Window,
	width: i32,
	height: i32,
	colors: Colors
}


impl CursedScreen {
	
	pub fn create() -> CursedScreen {
		let screen = curses::initscr();
		curses::noecho();
		curses::cbreak();
		screen.keypad(true);
		curses::curs_set(0);
		
		CursedScreen {
			colors: Colors::initialize(),
			width: screen.get_max_x(),
			height: screen.get_max_y(),
			screen,
		}
	}
}

impl Screen for CursedScreen {
	fn finalize(self) {
	
		self.screen.keypad(false);
		curses::echo();
		curses::nocbreak();
		curses::endwin();
	}
	
	fn update_size(&mut self) {
		let (y, x) = self.screen.get_max_yx();
		self.width = x;
		self.height = y;
	}
	
	fn write_screen_buffer(&self, buffer: &ScreenBuffer, (dest_x, dest_y): (usize, usize), (src_x, src_y): (usize, usize), (width, height): (usize, usize)){
		for y in 0..height {
			self.screen.mv((height - 1  - (y + dest_y)) as i32, dest_x as i32);
			for x in 0..width {
				if let Some(cell) = buffer.get((x + src_x, y + src_y )){
					self.screen.attrset(self.colors.get(&cell));
					self.screen.addch(cell.ch);
				}
			}
		}
	}
	
	fn width(&self) -> usize {
		self.width as usize
	}
	fn height(&self) -> usize {
		self.height as usize
	}
	
	fn await_keyboard_input(&self) -> Option<GameInput>{
		Some(match self.screen.getch()? {
			Input::Character(' ') => Quit,
			Input::Unknown(3) => Quit,
			Input::Character('a') => TurnLeft,
			Input::Character('d') => TurnRight,
			Input::Character('w') => Forward,
			Input::Character('s') => Back,
			Input::Character('q') => MoveLeft,
			Input::Character('e') => MoveRight,
			_ => Nothing
		})
	}
	
}






