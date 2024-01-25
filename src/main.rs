use image::{ RgbImage, ImageBuffer, Rgb };
use std::f32::consts::{PI, TAU};
use std::rc::Rc;
use rand::Rng;

mod vec3f;

use vec3f::Vec3f;

const IMAGE_WIDTH: u32 = 1024;
const IMAGE_HEIGHT: u32 = 1024;
const VIEWPORT_WIDTH: f32 = 1.0;
const VIEWPORT_HEIGHT: f32 = 1.0;
const VIEWPORT_Z: f32 = -1.0;
const DELTA_X: f32 = VIEWPORT_WIDTH / IMAGE_WIDTH as f32;
const DELTA_Y: f32 = -VIEWPORT_HEIGHT / IMAGE_HEIGHT as f32;
const CORNER_X: f32 = -VIEWPORT_WIDTH / 2.0 + DELTA_X / 2.0;
const CORNER_Y: f32 = VIEWPORT_HEIGHT / 2.0 + DELTA_Y / 2.0;

#[derive(Debug, Default)]
struct Ray {
	origin: Vec3f,
	direction: Vec3f
}

#[derive(Debug)]
struct HitResult {
	point: Vec3f,
	normal: Vec3f,
	t: f32,
	material: Rc<dyn Material>
}

trait Material {
	fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> Option<(Vec3f, Ray)>;
}

impl std::fmt::Debug for dyn Material {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", "trait Material")
	}
}

#[derive(Debug)]
struct Sphere {
	center: Vec3f,
	radius: f32,
	material: Rc<dyn Material>
}

#[derive(Debug, Clone, Copy)]
struct Interval {
	min: f32,
	max: f32
}

impl Interval {
	fn new_empty() -> Self {
		Interval { min: std::f32::MAX, max: std::f32::MIN }
	}

	fn new_universe() -> Self {
		Interval { min: 0.0001, max: std::f32::MAX }
	}

	fn new(min: f32, max: f32) -> Self {
		Interval { min, max }
	}

	fn contains(&self, x: f32) -> bool {
		self.min <= x && x <= self.max
	}

	fn surrounds(&self, x: f32) -> bool {
		self.min < x && x < self.max
	}
}

impl Ray {
	fn at(&self, t: f32) -> Vec3f {
		self.direction * t + self.origin
	}
}

impl Sphere {
	fn new(center: Vec3f, radius: f32, material: Rc<dyn Material>) -> Sphere {
		Sphere { center, radius, material }
	}

	fn random_vec3f() -> Vec3f {
		// TODO: This isn't uniform on the sphere

		let mut rng = rand::thread_rng();

		let theta = rng.gen_range(0.0 ..= PI);
		let phi = rng.gen_range(0.0 ..= TAU);

		Vec3f {
			x: theta.sin() * phi.cos(),
			y: theta.sin() * phi.sin(),
			z: theta.cos()
		}
	}
}

trait Hittable {
	fn hit(&self, interval: &Interval, ray: &Ray) -> Option<HitResult>;
}

impl Hittable for Sphere {
	fn hit(&self, interval: &Interval, ray: &Ray) -> Option<HitResult> {
		let oc = ray.origin - self.center;
		let a = ray.direction.lengthsq();
		let half_b = Vec3f::dot(&oc, &ray.direction);
		let c = oc.lengthsq() - self.radius * self.radius;
		let d = half_b * half_b - a * c;

		if d < 0.0 { return None }

		let sqrtd = d.sqrt();

		let mut root = (-half_b - sqrtd) / a;

		if !interval.surrounds(root) {
			root = (-half_b + sqrtd) / a;
			if !interval.surrounds(root) {
				return None
			}
		}

		let point = ray.at(root);

		Some(HitResult {
			t: root,
			point: point,
			normal: (point - self.center) / self.radius,
			material: self.material.clone()
		})
	}
}

struct HittableGroup {
	group: Vec<Box<dyn Hittable>>
}

impl HittableGroup {
	fn new() -> Self {
		HittableGroup { group: Vec::new() }
	}
}

impl Hittable for HittableGroup {
	fn hit(&self, interval: &Interval, ray: &Ray) -> Option<HitResult> {
		let mut nearest = interval.max;
		let mut nearest_result = None;

		for hittable in self.group.iter() {
			if let Some(hit_result) = hittable.hit(&Interval::new(interval.min, nearest), ray) {
				nearest = hit_result.t;
				nearest_result = Some(hit_result);
			}
		}

		nearest_result
	}
}

#[derive(Debug, Clone, Copy)]
struct Diffuse {
	albedo: Vec3f
}

impl Diffuse {
	fn new(albedo: Vec3f) -> Diffuse {
		Diffuse { albedo }
	}
}

impl Material for Diffuse {
	fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> Option<(Vec3f, Ray)> {
		let direction = hit_result.normal + Vec3f::rand();

		Some((
			self.albedo,
			Ray {
				origin: hit_result.point,
				direction: direction
			}
		))
	}
}

struct Metal {
	albedo: Vec3f
}

impl Metal {
	fn new(albedo: Vec3f) -> Metal {
		Metal { albedo }
	}
}

impl Material for Metal {
	fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> Option<(Vec3f, Ray)> {
		let reflected = Vec3f::reflect(&ray.direction, &hit_result.normal);
		Some((
			self.albedo,
			Ray {
				origin: hit_result.point,
				direction: reflected
			}
		))
	}
}

fn sample_square() -> Vec3f {
	let rx = -0.5 + rand::thread_rng().gen::<f32>();
	let ry = -0.5 + rand::thread_rng().gen::<f32>();
	Vec3f::new(rx * DELTA_X, ry * DELTA_Y, 0.0)
}

fn find_initial_ray(pixel_x: u32, pixel_y: u32) -> Ray {
	let viewport_x: f32 = CORNER_X + DELTA_X * pixel_x as f32;
	let viewport_y: f32 = CORNER_Y + DELTA_Y * pixel_y as f32;

	let sample_off = sample_square();

	Ray {
		origin: Vec3f::new(0.0, 0.0, 0.0),
		direction: (Vec3f::new(viewport_x, viewport_y, VIEWPORT_Z) + sample_off).unit()
	}
}

fn raycast(ray: &Ray, world: &HittableGroup, depth: u32) -> Vec3f {

	if depth == 0 {
		return Vec3f::new(0.0, 0.0, 0.0);
	}

	if let Some(hit_result) = world.hit(&Interval::new_universe(), ray) {
		if let Some((attenuation, scattered)) = hit_result.material.scatter(ray, &hit_result) {
			return attenuation * raycast(&scattered, world, depth - 1);
		}

		return Vec3f::new(0.0, 0.0, 0.0);
	}

	const BLUE: Vec3f = Vec3f { x: 0.5, y: 0.7, z: 1.0 };
	let a = 0.5 * (ray.direction.y + 1.0);

	BLUE * a + (1.0 - a)
}

fn main() {
	let mut objects = HittableGroup::new();

	let diffuse: Rc<dyn Material> = Rc::new(Diffuse::new(Vec3f::new(0.98 / 2.0, 0.70 / 2.0, 0.651 / 2.0)));
	let purple_diffuse: Rc<dyn Material> = Rc::new(Diffuse::new(Vec3f::new(0.54, 0.44, 0.60)));
	let metal: Rc<dyn Material> = Rc::new(Metal::new(Vec3f::new(0.8, 0.8, 0.8)));
	let gold: Rc<dyn Material> = Rc::new(Metal::new(Vec3f::new(255.0 / 255.0, 215.0 / 255.0, 0.0)));

	objects.group.push(Box::new(
		Sphere::new(
			Vec3f::new(-0.21, -0.1, -1.0),
			0.10,
			Rc::clone(&metal)
		 )
	));

	objects.group.push(Box::new(
		Sphere::new(
			Vec3f::new(0.0, -0.1, -1.0),
			0.10,
			Rc::clone(&gold)
		 )
	));

	objects.group.push(Box::new(
		Sphere::new(
			Vec3f::new(0.21, -0.1, -1.0),
			0.10,
			Rc::clone(&purple_diffuse)
		 )
	));

	objects.group.push(Box::new(
		Sphere::new(
			Vec3f::new(0.0, -20.2, -1.0),
			20.0,
			Rc::clone(&diffuse)
		 )
	));

	let mut buffer: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

	for (x, y, pixel) in buffer.enumerate_pixels_mut() {

		let mut color = Vec3f::new(0.0, 0.0, 0.0);

		for _ in 0 .. 512 {
			let ray = find_initial_ray(x, y);
			let value = raycast(&ray, &objects, 10);
			color = color + value;
		}

		color = color / 512.0;

		let r = (color.x * 255.0) as u8;
		let g = (color.y * 255.0) as u8;
		let b = (color.z * 255.0) as u8;

		*pixel = Rgb([r, g, b]);
	}

	buffer.save("render.png").unwrap();
}