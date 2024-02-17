use crate::geometry::{HitResult, Ray};
use crate::texture::Texture;
use crate::vec3f::Vec3f;

use std::sync::Arc;

pub trait Material: Send + Sync {
	fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> Option<(Vec3f, Ray)>;
	fn emit(&self, u: f64, v: f64) -> Vec3f {
		Vec3f::new(0.0, 0.0, 0.0)
	}
}

pub struct Diffuse {
	albedo: Arc<dyn Texture>,
}

impl Diffuse {
	pub const fn new(texture: Arc<dyn Texture>) -> Diffuse {
		Diffuse { albedo: texture }
	}
}

impl Material for Diffuse {
	fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> Option<(Vec3f, Ray)> {
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

pub struct Metal {
	albedo: Arc<dyn Texture>,
}

impl Metal {
	pub const fn new(texture: Arc<dyn Texture>) -> Metal {
		Metal { albedo: texture }
	}
}

impl Material for Metal {
	fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> Option<(Vec3f, Ray)> {
		let reflected = Vec3f::reflect(ray.direction, hit_result.normal);
		Some((
			self.albedo
				.value(hit_result.u, hit_result.v, &hit_result.point),
			Ray {
				origin: hit_result.point,
				direction: reflected,
			},
		))
	}
}

pub struct DiffuseLight {
	emit: Arc<dyn Texture>
}

impl DiffuseLight {
	pub fn new(emit: Arc<dyn Texture>) -> DiffuseLight {
		DiffuseLight { emit }
	}
}

impl Material for DiffuseLight {
	fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> Option<(Vec3f, Ray)> {
		None
	}

	fn emit(&self, u: f64, v: f64) -> Vec3f {
		self.emit.value(u, v, &Vec3f::new(0.0, 0.0, 0.0))
	}
}
