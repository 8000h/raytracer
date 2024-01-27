use geometry::Hittable;
use image::{ RgbImage, ImageBuffer, Rgb };
use std::ops::DerefMut;
use std::rc::Rc;
use std::sync::Arc;
use std::thread::{self, JoinHandle};

mod vec3f;
mod camera;
mod geometry;
mod material;

use crate::vec3f::Vec3f;
use crate::camera::Camera;
use crate::geometry::{ HittableGroup, Sphere };
use crate::material::{ Material, Diffuse, Metal };

const IMAGE_WIDTH: u32 = 1024;
const IMAGE_HEIGHT: u32 = 1024;
const IMAGE_AREA: usize = (IMAGE_WIDTH * IMAGE_HEIGHT) as usize;
const SAMPLES: usize = 512;
const THREADS: usize = 12;

#[derive(Copy, Clone)]
struct Pixel {
	r: u8,
	g: u8,
	b: u8
}

struct ImageFragment {
	pixels: Vec<Pixel>
}

impl ImageFragment {
	fn new(pixel_count: u32) -> Self {
		ImageFragment { pixels: Vec::with_capacity(pixel_count as usize) }
	}
}

fn render(world: Arc<HittableGroup>, camera: &Camera, row_start: u32, row_end: u32) -> ImageFragment {
	let rows = row_end - row_start;
	let mut fragment = ImageFragment::new(rows * IMAGE_WIDTH);

	for row in 0 .. rows {
		let y = row * IMAGE_HEIGHT;
		for x in 0 .. IMAGE_WIDTH {
			let mut color = Vec3f::new(0.0, 0.0, 0.0);
			for _ in 0 .. SAMPLES {
				let ray = camera.intial_ray(x, y);
				let value = camera.raycast(&ray, &world, 10);
				color = color + value;
			}

			color = color / SAMPLES as f32;

			fragment.pixels.push(Pixel {
				r: (color.x * 255.0) as u8,
				g: (color.y * 255.0) as u8,
				b: (color.z * 255.0) as u8
			});
		}
	}

	fragment
}

fn main() {
	let mut scene: Arc<HittableGroup> = Arc::new(HittableGroup::new());

	let diffuse: Arc<dyn Material + Send + Sync> = Arc::new(Diffuse::new(Vec3f::new(0.98 / 2.0, 0.70 / 2.0, 0.651 / 2.0)));
	let purple_diffuse: Arc<dyn Material + Send + Sync> = Arc::new(Diffuse::new(Vec3f::new(0.54, 0.44, 0.60)));
	let metal: Arc<dyn Material + Send + Sync> = Arc::new(Metal::new(Vec3f::new(0.8, 0.8, 0.8)));
	let gold: Arc<dyn Material + Send + Sync> = Arc::new(Metal::new(Vec3f::new(255.0 / 255.0, 215.0 / 255.0, 0.0)));

	scene.group.push(Arc::new(
		Sphere::new(
			Vec3f::new(-0.21, -0.1, -1.0),
			0.10,
			Arc::clone(&metal)
		 )
	));

	scene.group.push(Arc::new(
		Sphere::new(
			Vec3f::new(0.0, -0.1, -1.0),
			0.10,
			Arc::clone(&gold)
		 )
	));

	scene.group.push(Arc::new(
		Sphere::new(
			Vec3f::new(0.21, -0.1, -1.0),
			0.10,
			Arc::clone(&purple_diffuse)
		 )
	));

	scene.group.push(Arc::new(
		Sphere::new(
			Vec3f::new(0.0, -20.2, -1.0),
			20.0,
			Arc::clone(&diffuse)
		 )
	));

	let camera = Camera::new(
		Vec3f::new(0.0, 0.0, 0.0),
		Vec3f::new(0.0, 0.0, -1.0),
		70.0,
		IMAGE_WIDTH as u32,
		IMAGE_HEIGHT as u32
	);


	let mut threads: Vec<JoinHandle<ImageFragment>> = Vec::new();

	for _ in 0 .. THREADS {
		threads.push(std::thread::spawn(|| {
			render(Arc::clone(&scene), &camera, 0, 10)
		}));
	}

	let fragments = threads.into_iter().map(|handle| handle.join().unwrap());

	let mut buffer: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

	for (x, y, pixel) in buffer.enumerate_pixels_mut() {
		*pixel = Rgb([0, 0, 0]);
	}

	buffer.save("render.png").unwrap();
}