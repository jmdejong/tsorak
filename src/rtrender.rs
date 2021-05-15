
use crate::{
	screenbuffer::ScreenBuffer,
	scene::{Scene, ShapeObject},
	camera::Camera,
	util::{
		Vector2,
		Vector3,
		Point2
	}
};



pub fn render_raycast(target: &mut ScreenBuffer, scene: &Scene, camera: &Camera){

	let mut depth_buffer: Vec<f32> = Vec::new();
	for (x, direction2d) in camera.calculate_hor_rays(target.width()).into_iter().enumerate() {
		let shapes: Vec<ShapeObject> = scene.shapes_on_ray2d(Point2::new(camera.position.x, camera.position.y), direction2d);
		depth_buffer.clear();
		depth_buffer.resize(target.height(), f32::INFINITY);
		for shape in shapes {
// 			println!("{:?} {:?}", direction2d, shape);
			// todo: don't cast all rays for short objects
			for (y, angle_vert) in camera.calculate_vert_angles(target.height()).into_iter().enumerate() {
				let direction : Vector3 = direction2d.extend(angle_vert);
				if let Some(hit) = shape.intersect_ray(camera.position, direction){
// 					println!("hit: {:?}", hit);
					if hit.distance > depth_buffer[y] {
						continue;
					}
					depth_buffer[y] = hit.distance;
					target.set((x, y), Some(hit.brush));
				}
			}
		}
	}
}
