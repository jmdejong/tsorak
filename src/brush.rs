

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

impl Color {
	pub fn truncate(&self) -> Color {
		if self.0 == 8 {
			Color(7)
		} else {
			Color(self.0 % 8)
		}
	}
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Style {
	pub fg: Color,
	pub bg: Color
}

impl Style {
	pub fn new(fg: Color, bg: Color) -> Style {
		Style {fg, bg}
	}
	pub fn truncate(&self) -> Style {
		Style{fg: self.fg.truncate(), bg: self.bg.truncate()}
	}
}


impl Default for Style {
	fn default() -> Self {
		Self{ fg: Color(7), bg: Color(0)}
	}
}

pub fn style(fg: u8, bg: u8) -> Style{
	Style{fg: Color(fg), bg: Color(bg)}
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Brush {
	pub ch: char,
	pub style: Style,
	pub backupstyle: Style
}

pub fn brush(ch: char, fg: u8, bg: u8) -> Brush {
	let style = Style{fg: Color(fg), bg: Color(bg)};
	Brush {ch, style, backupstyle: style.truncate()}
}

impl Default for Brush {
	fn default() -> Self {
		Self{ ch: ' ', style: Style::default(), backupstyle: Style::default()}
	}
}
