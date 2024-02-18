use std::sync::Arc;

use crate::geometry::{Aabb3d, HitResult, Hittable, Interval, Ray, Vec3f};
use crate::material::Material;

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
