mod diffuse;
mod metal;
mod texture;

pub use self::diffuse::*;
pub use self::metal::*;
pub use self::texture::*;

use crate::geometry::{HitResult, Ray, Vec3f};

pub trait Material: Send + Sync {
	fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> Option<(Vec3f, Ray)>;
	fn emit(&self, _: f64, _: f64) -> Vec3f {
		Vec3f::new(0.0, 0.0, 0.0)
	}
}
