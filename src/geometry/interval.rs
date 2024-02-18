use crate::geometry::{Aabb3d, Vec3f};
use crate::material::Material;

use std::sync::Arc;

#[derive(Debug, Clone, Copy)]
pub struct Interval {
	pub min: f64,
	pub max: f64,
}

impl Interval {
	pub fn new_ray() -> Self {
		Interval {
			// Add a bit of padding to prevent "shadow acne"
			min: 0.000001,
			max: std::f64::MAX,
		}
	}

	pub fn new(min: f64, max: f64) -> Self {
		Interval { min, max }
	}

	pub fn from_intervals(i0: &Interval, i1: &Interval) -> Interval {
		Interval {
			min: f64::min(i0.min, i1.min),
			max: f64::max(i0.max, i1.max),
		}
	}

	pub fn expand(interval: &Interval, delta: f64) -> Interval {
		Interval {
			min: interval.min - delta / 2.0,
			max: interval.max + delta / 2.0,
		}
	}

	pub fn size(&self) -> f64 {
		self.max - self.min
	}

	pub fn surrounds(&self, x: f64) -> bool {
		self.min < x && x < self.max
	}

	pub fn contains(&self, x: f64) -> bool {
		self.min <= x && self.max <= self.max
	}
}

pub trait Hittable: Send + Sync {
	fn hit(&self, interval: &Interval, ray: &Ray) -> Option<HitResult>;
	fn bounds(&self) -> &Aabb3d;
}

#[derive(Debug, Default)]
pub struct Ray {
	pub origin: Vec3f,
	pub direction: Vec3f,
}

impl Ray {
	pub fn at(&self, t: f64) -> Vec3f {
		self.direction * t + self.origin
	}
}

pub struct HitResult {
	pub point: Vec3f,
	pub normal: Vec3f,
	pub t: f64,
	pub material: Arc<dyn Material>,
	pub u: f64,
	pub v: f64,
}

pub struct HittableGroup {
	group: Vec<Box<dyn Hittable>>,
	bounds: Aabb3d,
}

impl HittableGroup {
	pub fn new() -> Self {
		HittableGroup {
			group: Vec::new(),
			bounds: Aabb3d::default(),
		}
	}

	pub fn add(&mut self, hittable: Box<dyn Hittable>) {
		self.bounds = Aabb3d::from_bounds(&self.bounds, hittable.bounds());
		self.group.push(hittable);
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

	fn bounds(&self) -> &Aabb3d {
		&self.bounds
	}
}
