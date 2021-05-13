
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



fn main() {
	let screen = CursedScreen::create();
// 	let mut screen = DebugScreen(80, 20);
	let (w, h) = screen.get_size();
	let mut buffer = ScreenBuffer::new(w as usize, h as usize);
	
	let ratio = w as f32 / 2.0 / h as f32;
	let vert_fov_deg: f32 = 75.0;
	let vert_side = f32::tan(vert_fov_deg / 2.0 / 180.0 * std::f32::consts::PI);
	let mut camera = Camera::new(vert_side * ratio, vert_side); 
	
	let mut player = Player{pos: Point3::new(2.0, -10.0, 1.7), dir: 0};
	let wood_style = Style{fg: Color(7), bg: Color(3)};
	let stone_style = Style{fg: Color(7), bg: Color(8)};
	let scene = Scene::new(&[
		(Shape::HorPlane(-0.0), brush('.', Style{fg: Color(2), bg: Color(0)})),
		(wall((0.0, 0.0, 0.0), (5.0, 0.0, 3.0)), brush('1', wood_style)),
		(wall((0.0, 5.0, 0.0), (5.0, 5.0, 3.0)), brush('2', wood_style)),
		(wall((0.0, 0.0, 0.0), (0.0, 5.0, 3.0)), brush('3', stone_style)),
		(wall((5.0, 0.0, 0.0), (5.0, 5.0, 3.0)), brush('4', stone_style))
	]);
	let mut input = Input::Nothing;
	while input != Input::Quit {
		player.domove(input);
		camera.move_view(player.pos, player.view_angle());
		buffer.clear();
		rtrender::render_raycast(&mut buffer, &scene, &camera);
		screen.write_screen_buffer(&buffer, (0, 0), (0, 0), screen.get_size());
		input = screen.await_keyboard_input().unwrap();
	}
	
	screen.finalize();
	println!("input: ({:?}); w: {}, h: {}", input, w, h)
}
