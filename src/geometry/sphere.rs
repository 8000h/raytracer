use std::sync::Arc;

use crate::geometry::{Aabb3d, HitResult, Hittable, Interval, Ray, Vec3f};
use crate::material::Material;

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
