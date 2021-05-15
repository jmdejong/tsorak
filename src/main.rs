
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



use util::{Point3};
use player::Player;
use screenbuffer::ScreenBuffer;
use cursedscreen::CursedScreen;
use brush::{brush};
use camera::Camera;
use input::Input;
use screen::{Screen, DebugScreen};
use texture::{Texture};
use gamefield::{GameField, GameTile, TileShape};




fn main() {
	let screen = CursedScreen::create();
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
		',' => GameTile{floor: Some(brush(';', 2, 0)), shape: TileShape::Open, ceiling: None, accessible: true},
		'.' => GameTile{floor: Some(brush(':', 3, 0)), shape: TileShape::Open, ceiling: None, accessible: true},
		'~' => GameTile{floor: None, shape: TileShape::Open, ceiling: None, accessible: false},
		'#' => GameTile{
			floor: Some(brush('.', 7, 0)),
			ceiling: None,
			accessible: false,
			shape: TileShape::Block {
				height: 3.0,
				tex1: Texture::Flat(brush('#', 7, 8)),
				tex2: Texture::Flat(brush('#', 8, 7))
			}
		}
	};
	
	
	GameField::new(32, 32, &fieldgrid, mapping)
}
