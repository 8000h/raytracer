use rand::Rng;

use crate::geometry::{Hittable, HittableGroup, Interval, Ray, Vec3f};

pub struct Camera {
	pub background: Vec3f,
	pub position: Vec3f,
	pub dir: Vec3f,

	pixel_dx: Vec3f,
	pixel_dy: Vec3f,
	pixel_corner: Vec3f,
}

impl Camera {
	pub fn new(
		background: Vec3f,
		position: Vec3f,
		lookat: Vec3f,
		fov: f64,
		image_width: u32,
		image_height: u32,
	) -> Camera {
		let theta = fov.to_radians();
		let h = (theta / 2.0).tan();
		let viewport_height = 2.0 * h;
		let viewport_width = viewport_height; // Assuming 1:1 aspect ratio

		// Calculate basis vectors for the camera
		let up = Vec3f::new(0.0, 1.0, 0.0);
		let cz = (position - lookat).unit();
		let cx = Vec3f::cross(&up, &cz).unit();
		let cy = Vec3f::cross(&cx, &cz);

		// Calculate the vectors for the viewport
		let vx = cx * viewport_width;
		let vy = cy * viewport_height;

		let pixel_dx = vx / image_width as f64;
		let pixel_dy = vy / image_height as f64;

		let viewport_corner = (position - cz) - vx / 2.0 - vy / 2.0;
		let pixel_corner = viewport_corner + pixel_dx / 2.0 + pixel_dy / 2.0;

		let dir = lookat - position;

		Camera {
			background,
			dir,
			position,
			pixel_dx,
			pixel_dy,
			pixel_corner,
		}
	}

	fn sample_square(&self) -> Vec3f {
		let rx = -0.5 + rand::thread_rng().gen::<f64>();
		let ry = -0.5 + rand::thread_rng().gen::<f64>();

		self.pixel_dx * rx + self.pixel_dy * ry
	}

	pub fn intial_ray(&self, pixel_x: u32, pixel_y: u32) -> Ray {
		let point =
			self.pixel_corner + (self.pixel_dx * pixel_x as f64) + (self.pixel_dy * pixel_y as f64);

		let sample = self.sample_square();

		Ray {
			origin: self.position,
			direction: ((point - self.position) + sample).unit(),
		}
	}

	#[allow(unreachable_code)]
	pub fn raycast(&self, ray: &Ray, world: &HittableGroup, depth: u32) -> Vec3f {
		if depth == 0 {
			//return Vec3f::new(2.0, 2.0, 2.0);
			return self.background;
		}

		if let Some(hit_result) = world.hit(&Interval::new_ray(), ray) {
			let emitted = hit_result.material.emit(hit_result.u, hit_result.v);

			if let Some((attenuation, scattered)) = hit_result.material.scatter(ray, &hit_result) {
				let scatter = attenuation * self.raycast(&scattered, world, depth - 1);
				return emitted + scatter;
			} else {
				return emitted;
			}
		}

		return self.background;

		let blue: Vec3f = Vec3f {
			x: 0.5,
			y: 0.7,
			z: 1.0,
		};

		let a = 0.5 * (ray.direction.y + 1.0);

		blue * a + (1.0 - a)
	}
}
