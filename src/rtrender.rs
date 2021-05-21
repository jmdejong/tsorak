
use crate::{
	screenbuffer::ScreenBuffer,
	scene::{Scene, ShapeObject, Column, Hit},
	camera::Camera,
	util::{
		Vector3,
		Point2
	}
};



pub fn render_raycast(target: &mut ScreenBuffer, scene: &Scene, camera: &Camera){

	let mut depth_buffer: Vec<f32> = Vec::new();
	let rot = camera.rotation();
	for (x, direction2d) in camera.calculate_hor_rays(target.width()).into_iter().enumerate() {
		let columns: Vec<Column> = scene.shapes_on_ray2d(Point2::new(camera.position.x, camera.position.y), direction2d, &rot);
		for (y, angle_vert) in camera.calculate_vert_angles(target.height()).into_iter().enumerate() {
			let mut possible_hit = scene.plane_intersections(camera.position, direction2d.extend(angle_vert));
			
			for column in columns.iter() {
				if let Some(Hit{distance, ..}) = possible_hit {
					if column.t > distance {
						break;
					}
				}
				if let Some(brush) = column.get_hit(camera.position.z, angle_vert) {
					possible_hit = Some(Hit{distance: column.t, brush});
				}
			}
			if let Some(hit) = possible_hit {
				target.set((x, y), Some(hit.brush));
			}
		}
	}
}
