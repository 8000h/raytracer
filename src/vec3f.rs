use rand::Rng;
use std::ops::{Add, Div, Index, Mul, Sub};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3f {
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

impl Vec3f {
	pub const fn new(x: f64, y: f64, z: f64) -> Vec3f {
		Vec3f { x, y, z }
	}

	pub fn rand() -> Vec3f {
		let mut rand = rand::thread_rng();
		Vec3f {
			x: rand.gen_range(-1.0..=1.0),
			y: rand.gen_range(-1.0..=1.0),
			z: rand.gen_range(-1.0..=1.0),
		}
		.unit()
	}

	pub fn dot(lhs: &Vec3f, rhs: &Vec3f) -> f64 {
		lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
	}

	pub fn cross(a: &Vec3f, b: &Vec3f) -> Vec3f {
		Vec3f {
			x: a.y * b.z - a.z * b.y,
			y: a.z * b.x - a.x * b.z,
			z: a.x * b.y - a.y * b.x,
		}
	}

	pub fn reflect(v: Vec3f, normal: Vec3f) -> Vec3f {
		let b = Vec3f::dot(&v, &normal);
		v - normal * 2.0 * b
	}

	pub fn length(&self) -> f64 {
		(self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
	}

	pub fn lengthsq(&self) -> f64 {
		self.x * self.x + self.y * self.y + self.z * self.z
	}

	pub fn unit(&self) -> Vec3f {
		let length = self.length();
		Vec3f {
			x: self.x / length,
			y: self.y / length,
			z: self.z / length,
		}
	}
}

impl Add for Vec3f {
	type Output = Vec3f;

	fn add(self, rhs: Vec3f) -> Vec3f {
		Vec3f::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
	}
}

impl Add<f64> for Vec3f {
	type Output = Vec3f;

	fn add(self, rhs: f64) -> Vec3f {
		Vec3f::new(self.x + rhs, self.y + rhs, self.z + rhs)
	}
}

impl Sub for Vec3f {
	type Output = Vec3f;

	fn sub(self, rhs: Vec3f) -> Vec3f {
		Vec3f::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
	}
}

impl Mul for Vec3f {
	type Output = Vec3f;

	fn mul(self, rhs: Vec3f) -> Vec3f {
		Vec3f::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
	}
}

impl Mul<f64> for Vec3f {
	type Output = Vec3f;

	fn mul(self, rhs: f64) -> Vec3f {
		Vec3f::new(self.x * rhs, self.y * rhs, self.z * rhs)
	}
}

impl Div<f64> for Vec3f {
	type Output = Vec3f;

	fn div(self, rhs: f64) -> Vec3f {
		Vec3f::new(self.x / rhs, self.y / rhs, self.z / rhs)
	}
}

impl Index<usize> for Vec3f {
	type Output = f64;

	fn index(&self, index: usize) -> &f64 {
		match index {
			0 => &self.x,
			1 => &self.y,
			2 => &self.z,
			_ => panic!("Vec3f access out of bounds"),
		}
	}
}
