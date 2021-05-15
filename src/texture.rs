

use crate::{
	brush::{Brush, Style},
	screenbuffer::ScreenBuffer
};







#[derive(Debug, Clone, PartialEq)]
pub enum Texture {
	Flat(Brush),
	Checker(Brush, Brush),
	Image(ScreenBuffer),
	Tilemap(ScreenBuffer, (f32, f32))
}

impl Texture {
	pub fn get(&self, u: f32, v: f32) -> Option<Brush> {
		Some(match self {
			Texture::Flat(brush) => *brush,
			Texture::Checker(a, b) => {
				let x = u.floor() as i32;
				let y = v.floor() as i32;
				if (x / 5 + y / 5) % 2 == 0 {
					*a
				} else {
					*b
				}
			}
			Texture::Image(buffer) => {
				buffer.getf((u * buffer.width() as f32, v * buffer.height() as f32))?
			}
			Texture::Tilemap(buffer, scale) => buffer.getf((u / scale.0, v / scale.1))?
		})
	}
}


pub fn flat(ch: char, style: Style) -> Texture {
	Texture::Flat(Brush{ch, style})
}






