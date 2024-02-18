use std::sync::Arc;

use crate::geometry::{HitResult, Ray, Vec3f};
use crate::material::{Material, Texture};

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
