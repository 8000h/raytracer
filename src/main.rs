use geometry::Hittable;
use image::{ImageBuffer, Rgb, RgbImage};
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use texture::CheckerTexture;

mod aabb3d;
mod camera;
mod geometry;
mod material;
mod mesh;
mod texture;
mod triangle;
mod vec3f;

use crate::camera::Camera;
use crate::geometry::{HittableGroup, Plane, Sphere};
use crate::material::{Diffuse, Metal, DiffuseLight};
use crate::mesh::load_mesh;
use crate::texture::{ImageTexture, SolidColor, Texture};
use crate::triangle::Triangle;
use crate::vec3f::Vec3f;

const IMAGE_WIDTH: u32 = 768;
const IMAGE_HEIGHT: u32 = 768;
const SAMPLES: usize = 444;
const THREADS: usize = 64;
const BACKGROUND: Vec3f = Vec3f {
	x: 0.0,
	y: 0.0,
	z: 0.0
};

#[derive(Copy, Clone)]
struct Pixel {
	r: u8,
	g: u8,
	b: u8,
}

struct ImageFragment {
	pixels: Vec<Pixel>,
}

impl ImageFragment {
	fn new(pixel_count: u32) -> Self {
		ImageFragment {
			pixels: Vec::with_capacity(pixel_count as usize),
		}
	}
}

fn update_progress(completed: u32) {
	let percent = f32::round(completed as f32 / IMAGE_HEIGHT as f32 * 100.0);
	print!("\r{}% complete...", percent);
	std::io::stdout().flush();
}

fn render(
	world: Arc<HittableGroup>,
	camera: Arc<Camera>,
	row_start: u32,
	row_end: u32,
	completed: Arc<Mutex<u32>>,
) -> ImageFragment {
	let rows = row_end - row_start;
	let mut fragment = ImageFragment::new(rows * IMAGE_WIDTH);

	for row in row_start..row_end {
		let y = row;
		for x in 0..IMAGE_WIDTH {
			let mut color = Vec3f::new(0.0, 0.0, 0.0);
			for _ in 0..SAMPLES {
				let ray = camera.intial_ray(x, y);
				let value = camera.raycast(&ray, &world, 5);
				color = color + value;
			}

			color = color / SAMPLES as f64;

			fragment.pixels.push(Pixel {
				r: (color.x * 255.0) as u8,
				g: (color.y * 255.0) as u8,
				b: (color.z * 255.0) as u8,
			});
		}

		{
			let mut counter = completed.lock().unwrap();
			*counter += 1;
			update_progress(*counter);
		}

	}

	fragment
}

/*
static white_diffuse: Diffuse = Diffuse::new(Vec3f::new(0.90, 0.90, 0.90));
static diffuse: Diffuse = Diffuse::new(Vec3f::new(0.98 / 2.0, 0.70 / 2.0, 0.651 / 2.0));
static purple_diffuse: Diffuse = Diffuse::new(Vec3f::new(0.54, 0.44, 0.60));
static metal: Metal = Metal::new(Vec3f::new(0.8, 0.8, 0.8));
static gold: Metal = Metal::new(Vec3f::new(255.0 / 255.0, 215.0 / 255.0, 0.0));
*/

fn scene_cube(scene: &mut HittableGroup) -> Arc<Camera> {
	let camera = Arc::new(Camera::new(
		BACKGROUND,
		Vec3f::new(0.0, 1.0, 0.0),
		Vec3f::new(0.0, 0.0, -5.0),
		70.0,
		IMAGE_WIDTH as u32,
		IMAGE_HEIGHT as u32,
	));

	let plane_texture = Arc::new(CheckerTexture::new(
		Vec3f::new(1.0, 1.0, 1.0),
		Vec3f::new(0.7, 0.0, 0.0),
		1.0,
	));

	let plane_diffuse = Arc::new(Diffuse::new(plane_texture));

	let gray_texture = Arc::new(SolidColor::new(Vec3f::new(0.8, 0.8, 0.8)));
	let gray_metal = Arc::new(Metal::new(gray_texture));

	let purple_texture = Arc::new(SolidColor::new(Vec3f::new(0.98, 0.70, 0.65)));
	let purple_diffuse = Arc::new(Diffuse::new(purple_texture));

	let ant_texture: Arc<dyn Texture> = Arc::new(ImageTexture::new("models/IS.png"));
	let ant_diffuse = Arc::new(Diffuse::new(Arc::clone(&ant_texture)));
	let ant_metal = Arc::new(Metal::new(ant_texture));

	let white_texture = Arc::new(SolidColor::new(Vec3f::new(10.0, 10.0, 10.0)));
	let white_light = Arc::new(DiffuseLight::new(white_texture));

	let ant = mesh::load_mesh("models/cube.obj", ant_metal);

	scene.add(Box::new(ant));

	scene.add(Box::new(Plane::new(
		Vec3f::new(1.0, 0.0, 0.0),
		Vec3f::new(0.0, 0.0, -1.0),
		Vec3f::new(0.0, -0.1, 0.0),
		plane_diffuse,
	)));

	/*
	scene.add(Box::new(Sphere::new(
		Vec3f::new(0.0, 1.5, -5.0),
		1.0,
		gray_metal,
	)));

	 */
	camera
}

fn scene_tank(scene: &mut HittableGroup) -> Arc<Camera> {
	let camera = Arc::new(Camera::new(
		BACKGROUND,
		Vec3f::new(-1.0, 6.0, 20.0),
		Vec3f::new(0.0, 0.0, -5.0),
		70.0,
		IMAGE_WIDTH as u32,
		IMAGE_HEIGHT as u32,
	));

	let plane_texture = Arc::new(CheckerTexture::new(
		Vec3f::new(0.75, 0.75, 0.75),
		Vec3f::new(103.0 / 255.0, 197.0 / 255.0, 211.0 / 255.0),
		0.5,
	));

	let plane_diffuse = Arc::new(Diffuse::new(plane_texture));

	let gray_texture = Arc::new(SolidColor::new(Vec3f::new(0.8, 0.8, 0.8)));
	let gray_metal = Arc::new(Metal::new(gray_texture));

	let purple_texture = Arc::new(SolidColor::new(Vec3f::new(0.98, 0.70, 0.65)));
	let purple_diffuse = Arc::new(Diffuse::new(purple_texture));

	let ant_texture: Arc<dyn Texture> = Arc::new(ImageTexture::new("models/IS.png"));
	let ant_diffuse = Arc::new(Diffuse::new(Arc::clone(&ant_texture)));
	let ant_metal = Arc::new(Metal::new(ant_texture));

	let white_texture = Arc::new(SolidColor::new(Vec3f::new(10.0, 10.0, 10.0)));
	let white_light = Arc::new(DiffuseLight::new(white_texture));

	let ant = mesh::load_mesh("models/IS.obj", ant_diffuse);

	scene.add(Box::new(ant));

	scene.add(Box::new(Plane::new(
		Vec3f::new(1.0, 0.0, 0.0),
		Vec3f::new(0.0, 0.0, -1.0),
		Vec3f::new(0.0, -0.1, 0.0),
		plane_diffuse,
	)));

	scene.add(Box::new(Sphere::new(
		Vec3f::new(1.0, 25.0, 10.0),
		7.5,
		white_light,
	)));

	camera
}

fn scene_ant(scene: &mut HittableGroup) -> Arc<Camera> {
	let camera = Arc::new(Camera::new(
		BACKGROUND,
		Vec3f::new(0.0, 2.0, 0.0),
		Vec3f::new(0.0, 0.0, -5.0),
		90.0,
		IMAGE_WIDTH as u32,
		IMAGE_HEIGHT as u32,
	));

	let plane_texture = Arc::new(CheckerTexture::new(
		Vec3f::new(1.0, 1.0, 1.0),
		Vec3f::new(0.7, 0.0, 0.0),
		1.0,
	));

	let plane_diffuse = Arc::new(Diffuse::new(plane_texture));

	let gray_texture = Arc::new(SolidColor::new(Vec3f::new(0.8, 0.8, 0.8)));
	let gray_metal = Arc::new(Metal::new(gray_texture));

	let purple_texture = Arc::new(SolidColor::new(Vec3f::new(0.98, 0.70, 0.65)));
	let purple_diffuse = Arc::new(Diffuse::new(purple_texture));

	let ant_texture: Arc<dyn Texture> = Arc::new(ImageTexture::new("models/ant.png"));
	let ant_diffuse = Arc::new(Diffuse::new(Arc::clone(&ant_texture)));
	let ant_metal = Arc::new(Metal::new(ant_texture));

	let white_texture = Arc::new(SolidColor::new(Vec3f::new(10.0, 10.0, 10.0)));
	let white_light = Arc::new(DiffuseLight::new(white_texture));

	let ant = mesh::load_mesh("models/ant.obj", purple_diffuse);

	scene.add(Box::new(ant));

	scene.add(Box::new(Plane::new(
		Vec3f::new(1.0, 0.0, 0.0),
		Vec3f::new(0.0, 0.0, -1.0),
		Vec3f::new(0.0, -0.1, 0.0),
		plane_diffuse,
	)));

	scene.add(Box::new(Sphere::new(
		Vec3f::new(1.0, 4.5, -3.5),
		2.0,
		white_light,
	)));

	camera
}

fn scene_spheres(scene: &mut HittableGroup) -> Arc<Camera> {
	let camera = Arc::new(Camera::new(
		BACKGROUND,
		Vec3f::new(0.0, 0.0, 0.0),
		Vec3f::new(0.0, 0.0, -1.0),
		70.0,
		IMAGE_WIDTH as u32,
		IMAGE_HEIGHT as u32,
	));

	let plane_texture = Arc::new(CheckerTexture::new(
		Vec3f::new(1.0, 1.0, 1.0),
		Vec3f::new(0.7, 0.0, 0.0),
		10.0,
	));

	let plane_diffuse = Arc::new(Diffuse::new(plane_texture));

	let gray_texture = Arc::new(SolidColor::new(Vec3f::new(0.8, 0.8, 0.8)));
	let gray_metal = Arc::new(Metal::new(gray_texture));

	let purple_texture = Arc::new(SolidColor::new(Vec3f::new(0.98, 0.70, 0.65)));
	let purple_diffuse = Arc::new(Diffuse::new(purple_texture));

	let white_texture = Arc::new(SolidColor::new(Vec3f::new(1.0, 1.0, 1.0)));
	let white_light = Arc::new(DiffuseLight::new(white_texture));

	scene.add(Box::new(Plane::new(
		Vec3f::new(1.0, 0.0, 0.0),
		Vec3f::new(0.0, 0.0, -1.0),
		Vec3f::new(0.0, -0.1, 0.0),
		plane_diffuse,
	)));

	scene.add(Box::new(Sphere::new(
		Vec3f::new(-0.4, 0.35, -1.0),
		0.2,
		gray_metal
	)));

	scene.add(Box::new(Sphere::new(
		Vec3f::new(0.25, 0.15, -1.0),
		0.2,
		white_light
	)));

	camera
}

fn main() {
	let mut scene = HittableGroup::new();

	let camera = scene_tank(&mut scene);

	let scene: Arc<HittableGroup> = Arc::new(scene);

	let mut threads: Vec<JoinHandle<ImageFragment>> = Vec::new();

	let rows_common = IMAGE_HEIGHT / (THREADS as u32);
	let rows_last = rows_common + IMAGE_HEIGHT % (THREADS as u32);
	let mut rows = [rows_common; THREADS];
	rows[THREADS - 1] = rows_last;

	println!("Rendering...");

	let completed = Arc::new(Mutex::new(0u32));

	let mut start = 0;
	for i in 0..THREADS {
		let new_scene = Arc::clone(&scene);
		let new_camera = Arc::clone(&camera);
		let new_counter = Arc::clone(&completed);
		let end = start + rows[i];

		threads.push(std::thread::spawn(move || {
			render(new_scene, new_camera, start, end, new_counter)
		}));

		start = end;
	}

	let fragments: Vec<ImageFragment> = threads
		.into_iter()
		.map(|handle| handle.join().unwrap())
		.collect();

	let mut buffer: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

	let mut x = 0;
	let mut y = 0;
	for fragment in fragments {
		for pixel in fragment.pixels {
			buffer.put_pixel(x, y, Rgb([pixel.r, pixel.g, pixel.b]));

			x += 1;
			if x == IMAGE_WIDTH {
				x = 0;
				y += 1;
			}
		}
	}

	buffer.save("render.png").unwrap();
}
