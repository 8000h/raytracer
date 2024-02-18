use std::sync::Arc;

use crate::geometry::{HitResult, Ray, Vec3f};
use crate::material::{Material, Texture};

pub struct Diffuse {
	albedo: Arc<dyn Texture>,
}

impl Diffuse {
	pub const fn new(texture: Arc<dyn Texture>) -> Diffuse {
		Diffuse { albedo: texture }
	}
}

impl Material for Diffuse {
	fn scatter(&self, _: &Ray, hit_result: &HitResult) -> Option<(Vec3f, Ray)> {
		let direction = hit_result.normal + Vec3f::rand();

		Some((
			self.albedo
				.value(hit_result.u, hit_result.v, &hit_result.point),
			Ray {
				origin: hit_result.point,
				direction: direction,
			},
		))
	}
}

pub struct DiffuseLight {
	emit: Arc<dyn Texture>,
}

impl DiffuseLight {
	pub fn new(emit: Arc<dyn Texture>) -> DiffuseLight {
		DiffuseLight { emit }
	}
}

impl Material for DiffuseLight {
	fn scatter(&self, _: &Ray, _: &HitResult) -> Option<(Vec3f, Ray)> {
		None
	}

	fn emit(&self, u: f64, v: f64) -> Vec3f {
		self.emit.value(u, v, &Vec3f::new(0.0, 0.0, 0.0))
	}
}
