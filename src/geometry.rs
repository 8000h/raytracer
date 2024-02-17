use crate::aabb3d::Aabb3d;
use crate::material::Material;
use crate::vec3f::Vec3f;

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

pub struct Sphere {
	center: Vec3f,
	radius: f64,
	material: Arc<dyn Material>,
	bounds: Aabb3d,
}

impl Sphere {
	pub fn new(center: Vec3f, radius: f64, material: Arc<dyn Material>) -> Sphere {
		let rv = Vec3f::new(radius, radius, radius);
		let bounds = Aabb3d::from_corners(center - rv, center + rv);

		Sphere {
			center,
			radius,
			material,
			bounds,
		}
	}
}

impl Hittable for Sphere {
	fn hit(&self, interval: &Interval, ray: &Ray) -> Option<HitResult> {
		let oc = ray.origin - self.center;
		let a = ray.direction.lengthsq();
		let half_b = Vec3f::dot(&oc, &ray.direction);
		let c = oc.lengthsq() - self.radius * self.radius;
		let d = half_b * half_b - a * c;

		if d < 0.0 {
			return None;
		}

		let sqrtd = d.sqrt();

		let mut root = (-half_b - sqrtd) / a;

		if !interval.surrounds(root) {
			root = (-half_b + sqrtd) / a;
			if !interval.surrounds(root) {
				return None;
			}
		}

		let point = ray.at(root);

		Some(HitResult {
			t: root,
			point: point,
			normal: (point - self.center) / self.radius,
			material: Arc::clone(&self.material),
			u: 0.0,
			v: 0.0,
		})
	}

	fn bounds(&self) -> &Aabb3d {
		&self.bounds
	}
}

pub struct Plane {
	pub normal: Vec3f,
	pub xbasis: Vec3f,
	pub ybasis: Vec3f,
	pub point: Vec3f,
	pub material: Arc<dyn Material>,
	pub bounds: Aabb3d,
}

impl Plane {
	pub fn new(xbasis: Vec3f, ybasis: Vec3f, point: Vec3f, material: Arc<dyn Material>) -> Plane {
		let bounds = Aabb3d::from_corners(
			Vec3f::new(f64::MIN, f64::MIN, f64::MIN),
			Vec3f::new(f64::MAX, f64::MAX, f64::MAX),
		);

		let normal = Vec3f::cross(&xbasis, &ybasis).unit();

		Plane {
			normal,
			xbasis,
			ybasis,
			point,
			material,
			bounds,
		}
	}
}

impl Hittable for Plane {
	fn hit(&self, interval: &Interval, ray: &Ray) -> Option<HitResult> {
		let po = self.point - ray.origin;
		let t = Vec3f::dot(&po, &self.normal) / Vec3f::dot(&ray.direction, &self.normal);

		let hit_point = ray.at(t);
		let offset = hit_point - self.point;

		if interval.surrounds(t) {
			Some(HitResult {
				t: t,
				point: hit_point,
				normal: self.normal,
				material: Arc::clone(&self.material),
				u: Vec3f::dot(&self.xbasis, &offset),
				v: Vec3f::dot(&self.ybasis, &offset),
			})
		} else {
			None
		}
	}

	fn bounds(&self) -> &Aabb3d {
		&self.bounds
	}
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
