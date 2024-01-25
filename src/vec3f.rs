use std::ops::{ Add, Sub, Mul, Div };
use rand::Rng;

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3f {
	pub x: f32,
	pub y: f32,
	pub z: f32
}

impl Vec3f {
	pub fn new(x: f32, y: f32, z: f32) -> Vec3f {
		Vec3f { x, y, z }
	}

	pub fn rand() -> Vec3f {
		let mut rand = rand::thread_rng();
		Vec3f {
			x: rand.gen_range(-1.0 ..= 1.0),
			y: rand.gen_range(-1.0 ..= 1.0),
			z: rand.gen_range(-1.0 ..= 1.0)
		}.unit()
	}

	pub fn dot(left: &Vec3f, right: &Vec3f) -> f32 {
		left.x * right.x + left.y * right.y + left.z * right.z
	}

	pub fn reflect(v: &Vec3f, normal: &Vec3f) -> Vec3f {
		let b = Vec3f::dot(v, normal);
		*v - *normal * 2.0 * b
	}

	pub fn length(&self) -> f32 {
		(self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
	}

	pub fn lengthsq(&self) -> f32 {
		self.x * self.x + self.y * self.y + self.z * self.z
	}

	pub fn unit(&self) -> Vec3f {
		let length = self.length();
		Vec3f { x: self.x / length, y: self.y / length, z: self.z / length }
	}
}

impl Add for Vec3f {
	type Output = Vec3f;

	fn add(self, rhs: Vec3f) -> Vec3f {
		Vec3f::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
	}
}

impl Add<f32> for Vec3f {
	type Output = Vec3f;

	fn add(self, rhs: f32) -> Vec3f {
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
	type Output =  Vec3f;

	fn mul(self, rhs: Vec3f) -> Vec3f {
		Vec3f::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
	}
}

impl Mul<f32> for Vec3f {
	type Output = Vec3f;

	fn mul(self, rhs: f32) -> Vec3f {
		Vec3f::new(self.x * rhs, self.y * rhs, self.z * rhs)
	}
}

impl Div<f32> for Vec3f {
	type Output = Vec3f;

	fn div(self, rhs: f32) -> Vec3f {
		Vec3f::new(self.x / rhs, self.y / rhs, self.z / rhs)
	}
}