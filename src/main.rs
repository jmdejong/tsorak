
mod brush;
mod cursedscreen;
mod input;
mod screenbuffer;
mod rtrender;
mod scene;
mod camera;
mod util;
mod player;
mod screen;
mod texture;
mod grid;
mod gamefield;



use scene::{Scene, Shape, wall};
use util::{Point3, Rad};
use player::Player;
use screenbuffer::ScreenBuffer;
use cursedscreen::CursedScreen;
use brush::{brush, Style, Color};
use camera::Camera;
use input::Input;

use std::io;
use screen::{Screen, DebugScreen};
use texture::{Texture, flat};
use gamefield::{GameField, GameTile};





fn main() {
	let screen = CursedScreen::create();
// 	let mut screen = DebugScreen(80, 20);
	let (w, h) = screen.get_size();
	let mut buffer = ScreenBuffer::new(w as usize, h as usize);
	
	let ratio = w as f32 / 2.0 / h as f32;
	let vert_fov_deg: f32 = 75.0;
	let vert_side = f32::tan(vert_fov_deg / 2.0 / 180.0 * std::f32::consts::PI);
	let mut camera = Camera::new(vert_side * ratio, vert_side); 
	
	let mut player = Player{pos: Point3::new(10.0, 5.0, 1.7), dir: 0};
	let field = build_field();
	let scene = field.to_scene();
	let mut input = Input::Nothing;
	while input != Input::Quit {
		player.domove(input, &field);
		camera.move_view(player.pos, player.view_angle());
		buffer.fill(Some(brush(' ', 0, 0)));
		rtrender::render_raycast(&mut buffer, &scene, &camera);
		screen.write_screen_buffer(&buffer, (0, 0), (0, 0), screen.get_size());
		input = screen.await_keyboard_input().unwrap();
	}
	
	screen.finalize();
	println!("input: ({:?}); w: {}, h: {}", input, w, h)
}




fn build_field() -> GameField {

// 	let texbuf = ScreenBuffer::from_lines(3, 3, &["ABC", "DEF", "GHI"], &hashmap!{
// 		'A' => brush('A', 7, 5),
// 		'B' => brush('B', 7, 5),
// 		'C' => brush('C', 7, 5),
// 		'D' => brush('D', 7, 5),
// 		'E' => brush('E', 7, 5),
// 		'F' => brush('F', 7, 5),
// 		'G' => brush('G', 7, 5),
// 		'H' => brush('H', 7, 5),
// 		'I' => brush('I', 7, 5)
// 	});
// 	
	
	let fieldgrid = [
		"~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~",
		"~,,,,,,.,,,,,,,,,,,,,,,,,,,,,,,~",
		"~,,,,,,.,,,,,,,,,,,,,,,,,,,,,,,~",
		"~,,,,,,.,,,,,,,,,,,,,,,,,,,,,,,~",
		"~,,,,,,.,,,,,,,..,,,,,,,,,,,,,,~",
		"~,,,,,,.,,,,,,,..,,,,,,,,,,,,,,~",
		"~........................,,,,,,~",
		"~,,,,,,.,,,,,,,..,,,,,,,,,,,,,,~",
		"~,,,,,,.,##.##,..,,,,,,,,,,,,,,~",
		"~,,,,,,.,#...#,..,,,,,,,,,,,,,,~",
		"~,,,,,,.,#...#,..,,,,,,,,,,,,,,~",
		"~,,,,,,.,#...#,..,,,,,,,,,,,,,,~",
		"~,,,,,,.,#####,..,,,,,,,,,,,,,,~",
		"~,,,,,,.,,,,,,,,.,,,,,,,,,,,,,,~",
		"~,,,,,,.,,,,,,,,,,,,,,,,,,,,,,,~",
		"~,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,~",
		"~,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,~",
		"~,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,~",
		"~,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,~",
		"~,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,~",
		"~,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,~",
		"~,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,~",
		"~,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,~",
		"~,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,~",
		"~,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,~",
		"~,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,~",
		"~,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,~",
		"~,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,~",
		"~,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,~",
		"~,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,~",
		"~,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,~",
		"~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~",
	];
	
	let mapping = hashmap!{
		',' => GameTile{floor: Some(brush(';', 2, 0)), shapes: Vec::new(), ceiling: None, accessible: true},
		'.' => GameTile{floor: Some(brush(':', 3, 0)), shapes: Vec::new(), ceiling: None, accessible: true},
		'~' => GameTile{floor: None, shapes: Vec::new(), ceiling: None, accessible: false},
		'#' => GameTile{floor: Some(brush('.', 7, 0)), ceiling: None, accessible: false, shapes: vec![
			wall((0.0, 0.0, 0.0), (1.0, 0.0, 3.0), Texture::Flat(brush('#', 7, 8))),
			wall((1.0, 0.0, 0.0), (1.0, 1.0, 3.0), Texture::Flat(brush('#', 8, 7))),
			wall((1.0, 1.0, 0.0), (0.0, 1.0, 3.0), Texture::Flat(brush('#', 7, 8))),
			wall((0.0, 1.0, 0.0), (0.0, 0.0, 3.0), Texture::Flat(brush('#', 8, 7)))
		]}
	};
	
	
	GameField::new(32, 32, &fieldgrid, mapping)

// 	let wood_style = Style{fg: Color(7), bg: Color(3)};
// 	let stone_style = Style{fg: Color(7), bg: Color(8)};
// 	Scene::new(&[
// 		(Shape::HorPlane(-0.5), Texture::Flat(brush('~', 4, 0))),
// 		(Shape::HorPlane(0.0), Texture::Tilemap(tilebuf, (2.0, 2.0))),
// 		(wall((0.0, 0.0, 0.0), (5.0, 0.0, 3.0)), Texture::Image(texbuf)),
// 		(wall((0.0, 5.0, 0.0), (5.0, 5.0, 3.0)), flat('2', wood_style)),
// 		(wall((0.0, 0.0, 0.0), (0.0, 5.0, 3.0)), flat('3', stone_style)),
// 		(wall((5.0, 0.0, 0.0), (5.0, 5.0, 3.0)), flat('4', stone_style))
// 	])
}
