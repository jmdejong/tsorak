
use std::collections::HashMap;
use crate::{
	brush::{Brush, brush},
	scene::{Scene, ShapeObject, plane, wall},
	texture::Texture,
	screenbuffer::ScreenBuffer,
	grid::Grid
};


#[derive(Debug, Clone, PartialEq)]
pub struct GameTile {
	pub floor: Option<Brush>,
	pub shape: TileShape,
	pub ceiling: Option<Brush>,
	pub accessible: bool
}

#[derive(Debug, Clone, PartialEq)]
pub enum TileShape {
	Open,
	Block{height: f32, tex1: Texture, tex2: Texture},
// 	Sprite{tex: Texture, height: f32, width: f32}
}



pub struct GameField {
	tiles: Grid<char>,
	mapping: HashMap<char, GameTile>
}

impl GameField {
	
	pub fn new<S: AsRef<str>>(width: usize, height: usize, lines: &[S], mapping: HashMap<char, GameTile>) -> GameField {
		let tiles = Grid::from_lines(width, height, lines.into_iter().map(|s| s.as_ref().chars()), &' ');
		GameField {tiles, mapping}
	}
	
	fn add_wall(&self, shapes: &mut Vec<ShapeObject>, (x, y): (usize, usize), height: f32, (x0, y0): (usize, usize), (x1, y1): (usize, usize), (dx, dy): (isize, isize), tex: &Texture) {
		if let Some(TileShape::Block{height: h, tex1: _, tex2: _}) = self.tiles
				.get((x as isize + dx) as usize, (y as isize + dy) as usize)
				.and_then(|ch| self.mapping.get(ch))
				.map(|tile| &tile.shape){
			if *h >= height {
				return;
			}
		}
		shapes.push(wall(((x + x0) as f32, (y + y0) as f32, 0.0), ((x + x1) as f32, (y + y1) as f32, height), tex.clone()));
	}
	
	pub fn to_scene(&self) -> Scene {
		let mut shapes = vec![];
		let mut floor_buf = ScreenBuffer::new(self.tiles.width(), self.tiles.height());
		for (xy, typ) in self.tiles.iter_cells(){
			if let Some(val) = self.mapping.get(typ){
				floor_buf.set(xy, val.floor.clone());
				match &val.shape {
					TileShape::Open => (),
					TileShape::Block{height, tex1, tex2} => {
						self.add_wall(&mut shapes, xy, *height, (0,0), (1,0), (0,-1), &tex1);
						self.add_wall(&mut shapes, xy, *height, (1,0), (1,1), (1,0), &tex2);
						self.add_wall(&mut shapes, xy, *height, (1,1), (0,1), (0,1), &tex1);
						self.add_wall(&mut shapes, xy, *height, (0,1), (0,0), (-1,0), &tex2);
// 						wall((x,     y,     0.0), (x+1.0, y,     height), tex1),
// 						wall((x+1.0, y,     0.0), (x+1.0, y+1.0, height), tex2),
// 						wall((x+1.0, y+1.0, 0.0), (x,     y+1.0, height), tex1),
// 						wall((x,     y+1.0, 0.0), (x,     y,     height), tex2)
					}
				}
// 				for shape in val.shapes.iter() {
// 					shapes.push(shape.moved(Vector3::new(x as f32, y as f32, 0.0)));
// 				}
			}
		}
		Scene::new(
			&[
				plane(-0.5, Texture::Flat(brush('~', 4, 0))),
				plane(0.0, Texture::Tilemap(floor_buf, (1.0, 1.0)))
			],
			&shapes
		)
// 			(Shape::HorPlane(-0.5), Texture::Flat(brush('~', 4, 0))),
// 			(Shape::HorPlane(0.0), Texture::Tilemap(floor_buf, (2.0, 2.0))),
// 		])
	}
	
	pub fn is_accessible(&self, x: usize, y: usize) -> bool{
		self.tiles.get(x, y).and_then(|tile| self.mapping.get(tile)).map(|tile| tile.accessible) == Some(true)
	}
}
