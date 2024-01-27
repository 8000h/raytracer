use crate::vec3f::Vec3f;
use crate::geometry::{ Ray, HitResult };

pub trait Material {
	fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> Option<(Vec3f, Ray)>;
}

impl std::fmt::Debug for dyn Material {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", "trait Material")
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Diffuse {
	albedo: Vec3f
}

impl Diffuse {
	pub fn new(albedo: Vec3f) -> Diffuse {
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

pub struct Metal {
	albedo: Vec3f
}

impl Metal {
	pub fn new(albedo: Vec3f) -> Metal {
		Metal { albedo }
	}
}

impl Material for Metal {
	fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> Option<(Vec3f, Ray)> {
		let reflected = Vec3f::reflect(ray.direction, hit_result.normal);
		Some((
			self.albedo,
			Ray {
				origin: hit_result.point,
				direction: reflected
			}
		))
	}
}