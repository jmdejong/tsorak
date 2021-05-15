
use std::collections::HashMap;
use crate::{
	brush::{Brush, brush},
	scene::{Scene, ShapeObject, plane, wall},
	texture::Texture,
	screenbuffer::ScreenBuffer,
	grid::Grid,
	util::Vector3
};


pub struct GameTile {
	pub floor: Option<Brush>,
	pub shapes: Vec<ShapeObject>,
	pub ceiling: Option<Brush>,
	pub accessible: bool
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TileId(char);

pub struct GameField {
	tiles: Grid<char>,
	mapping: HashMap<char, GameTile>
}

impl GameField {
	
	pub fn new<S: AsRef<str>>(width: usize, height: usize, lines: &[S], mapping: HashMap<char, GameTile>) -> GameField {
		let tiles = Grid::from_lines(width, height, lines.into_iter().map(|s| s.as_ref().chars()), &' ');
		GameField {tiles, mapping}
	}
	
	pub fn to_scene(&self) -> Scene {
		let mut shapes = vec![plane(-0.5, Texture::Flat(brush('~', 4, 0)))];
		let mut floor_buf = ScreenBuffer::new(self.tiles.width(), self.tiles.height());
		for ((x, y), typ) in self.tiles.iter_cells(){
			if let Some(val) = self.mapping.get(typ){
				floor_buf.set((x, y), val.floor.clone());
				for shape in val.shapes.iter() {
					shapes.push(shape.moved(Vector3::new(x as f32, y as f32, 0.0)));
				}
			}
		}
		shapes.push(plane(0.0, Texture::Tilemap(floor_buf, (1.0, 1.0))));
		Scene::new(&shapes)
// 			(Shape::HorPlane(-0.5), Texture::Flat(brush('~', 4, 0))),
// 			(Shape::HorPlane(0.0), Texture::Tilemap(floor_buf, (2.0, 2.0))),
// 		])
	}
	
	pub fn is_accessible(&self, x: usize, y: usize) -> bool{
		self.tiles.get(x, y).and_then(|tile| self.mapping.get(tile)).map(|tile| tile.accessible) == Some(true)
	}
}
