use std::sync::Arc;

use crate::geometry::{Aabb3d, HitResult, Hittable, Interval, Ray, Vec3f};
use crate::material::Material;

use std::ops::{Add, Mul};

#[derive(Copy, Clone)]
pub struct Uv {
	pub u: f64,
	pub v: f64,
}

impl Add for Uv {
	type Output = Uv;

	fn add(self, rhs: Self) -> Self::Output {
		Uv {
			u: self.u + rhs.u,
			v: self.v + rhs.v,
		}
	}
}

impl Mul<f64> for Uv {
	type Output = Uv;

	fn mul(self, rhs: f64) -> Self::Output {
		Uv {
			u: self.u * rhs,
			v: self.v * rhs,
		}
	}
}

impl Uv {
	pub fn new(u: f64, v: f64) -> Uv {
		Uv { u, v }
	}
}

pub struct Triangle {
	pub a: Vec3f,
	pub ab: Vec3f,
	pub ac: Vec3f,

	pub uv_a: Uv,
	pub uv_b: Uv,
	pub uv_c: Uv,

	pub normal: Vec3f,
	pub material: Arc<dyn Material>,
	pub bounds: Aabb3d,
}

impl Triangle {
	pub fn new(
		a: Vec3f,
		b: Vec3f,
		c: Vec3f,
		uv_a: Uv,
		uv_b: Uv,
		uv_c: Uv,
		material: Arc<dyn Material>,
	) -> Triangle {
		let ab = b - a;
		let ac = c - a;

		let minx = f64::min(f64::min(a.x, b.x), c.x);
		let miny = f64::min(f64::min(a.y, b.y), c.y);
		let minz = f64::min(f64::min(a.z, b.z), c.z);

		let maxx = f64::max(f64::max(a.x, b.x), c.x);
		let maxy = f64::max(f64::max(a.y, b.y), c.y);
		let maxz = f64::max(f64::max(a.z, b.z), c.z);

		Triangle {
			a: a,
			ab: b - a,
			ac: c - a,
			uv_a: uv_a,
			uv_b: uv_b,
			uv_c: uv_c,
			normal: Vec3f::cross(&ab, &ac),
			material: material,
			bounds: Aabb3d::pad(&Aabb3d::from_corners(
				Vec3f::new(minx, miny, minz),
				Vec3f::new(maxx, maxy, maxz),
			)),
		}
	}
}

impl Hittable for Triangle {
	fn hit(&self, interval: &Interval, ray: &Ray) -> Option<HitResult> {
		let d = -Vec3f::dot(&self.normal, &ray.direction);

		// We either hit the back of the triangle, or the ray is parallel to the normal
		if d <= 0.0 {
			return None;
		}

		// Calculate the intersection of the ray onto the plane
		let ap = ray.origin - self.a;
		let t = Vec3f::dot(&ap, &self.normal) / d;

		// Check if the intersection is in the interval
		if !interval.contains(t) {
			return None;
		}

		// Find barycentric coordinates for triangle
		let e = Vec3f::cross(&(ray.direction * -1.0), &ap);
		let v = Vec3f::dot(&self.ac, &e) / d;

		if v < 0.0 || v > 1.0 {
			return None;
		}

		let w = -Vec3f::dot(&self.ab, &e) / d;

		// Check barycentric constraints
		if w < 0.0 || v + w > 1.0 {
			return None;
		}

		// Perform the delayed division
		let u = 1.0 - v - w;

		// U -> a
		// V -> b
		// W -> c

		let uv = self.uv_a * u + self.uv_b * v + self.uv_c * w;

		Some(HitResult {
			point: ray.at(t),
			normal: self.normal.unit(),
			t,
			material: Arc::clone(&self.material),
			u: uv.u,
			v: uv.v,
		})
	}

	fn bounds(&self) -> &Aabb3d {
		&self.bounds
	}
}
