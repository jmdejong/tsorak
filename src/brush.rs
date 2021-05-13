

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color(pub u8);
// pub enum Color{
// 	Black,
// 	Red,
// 	Green,
// 	Blue,
// 	Brown,
// 	Purple,
// 	DarkCyan,
// 	LightGray,
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Style {
	pub fg: Color,
	pub bg: Color
}

impl Style {
	pub fn new(fg: Color, bg: Color) -> Style {
		Style {fg, bg}
	}
}


impl Default for Style {
	fn default() -> Self {
		Self{ fg: Color(7), bg: Color(0)}
	}
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Brush {
	pub ch: char,
	pub style: Style
}

pub fn brush(ch: char, style: Style) -> Brush {
	Brush { ch, style}
}

impl Default for Brush {
	fn default() -> Self {
		Self{ ch: ' ', style: Style::default()}
	}
}
