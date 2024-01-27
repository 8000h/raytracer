use crate::vec3f::Vec3f;
use crate::material::Material;

use std::ops::DerefMut;
use std::sync::Arc;

#[derive(Debug, Clone, Copy)]
pub struct Interval {
	pub min: f32,
	pub max: f32
}

impl Interval {
	pub fn new_empty() -> Self {
		Interval { min: std::f32::MAX, max: std::f32::MIN }
	}

	pub fn new_universe() -> Self {
		Interval { min: 0.0001, max: std::f32::MAX }
	}

	pub fn new(min: f32, max: f32) -> Self {
		Interval { min, max }
	}

	pub fn contains(&self, x: f32) -> bool {
		self.min <= x && x <= self.max
	}

	pub fn surrounds(&self, x: f32) -> bool {
		self.min < x && x < self.max
	}
}

pub trait Hittable {
	fn hit(&self, interval: &Interval, ray: &Ray) -> Option<HitResult>;
}

#[derive(Debug, Default)]
pub struct Ray {
	pub origin: Vec3f,
	pub direction: Vec3f
}

impl Ray {
	pub fn at(&self, t: f32) -> Vec3f {
		self.direction * t + self.origin
	}
}

#[derive(Debug)]
pub struct HitResult {
	pub point: Vec3f,
	pub normal: Vec3f,
	pub t: f32,
	pub material: Arc<dyn Material>
}

pub struct Sphere {
	center: Vec3f,
	radius: f32,
	radiussq: f32,
	material: Arc<dyn Material + Send + Sync>
}

impl Sphere {
	pub fn new(center: Vec3f, radius: f32, material: Arc<dyn Material + Send + Sync>) -> Sphere {
		let radiussq = radius * radius;
		Sphere { center, radius, radiussq, material }
	}
}

impl Hittable for Sphere {
	fn hit(&self, interval: &Interval, ray: &Ray) -> Option<HitResult> {
		let oc = ray.origin - self.center;
		let a = ray.direction.lengthsq();
		let half_b = Vec3f::dot(&oc, &ray.direction);
		let c = oc.lengthsq() - self.radiussq;
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

pub struct HittableGroup {
	pub group: Vec<Arc<dyn Hittable + Send + Sync>>
}

impl HittableGroup {
	pub fn new() -> Self {
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