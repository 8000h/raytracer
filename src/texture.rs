use crate::vec3f::Vec3f;

use image::{DynamicImage, GenericImageView, Pixel, RgbImage};

pub trait Texture: Send + Sync {
	fn value(&self, u: f64, v: f64, point: &Vec3f) -> Vec3f;
}

pub struct SolidColor {
	pub value: Vec3f,
}

impl SolidColor {
	pub fn new(value: Vec3f) -> SolidColor {
		SolidColor { value }
	}
}

impl Texture for SolidColor {
	fn value(&self, u: f64, v: f64, point: &Vec3f) -> Vec3f {
		self.value
	}
}

pub struct CheckerTexture {
	pub even_color: Vec3f,
	pub odd_color: Vec3f,
	pub scale: f64,
}

impl CheckerTexture {
	pub fn new(even_color: Vec3f, odd_color: Vec3f, scale: f64) -> CheckerTexture {
		CheckerTexture {
			even_color,
			odd_color,
			scale,
		}
	}
}

impl Texture for CheckerTexture {
	fn value(&self, u: f64, v: f64, point: &Vec3f) -> Vec3f {
		let ix = f64::round(u * self.scale) as i32;
		let iy = f64::round(v * self.scale) as i32;

		if (ix + iy) % 2 == 0 {
			self.even_color
		} else {
			self.odd_color
		}
	}
}

pub struct ImageTexture {
	data: DynamicImage,
	width: u32,
	height: u32,
}

impl ImageTexture {
	pub fn new(path: &str) -> ImageTexture {
		let mut data = image::open(path).unwrap();
		let width = data.width();
		let height = data.height();

		ImageTexture {
			data,
			width,
			height,
		}
	}
}

impl Texture for ImageTexture {
	fn value(&self, u: f64, v: f64, point: &Vec3f) -> Vec3f {
		let mut px = f64::round(u * (self.width as f64 - 1.0)) as i32;
		let mut py = f64::round(v * (self.height as f64 - 1.0)) as i32;

		px = px.rem_euclid(self.width as i32);
		py = py.rem_euclid(self.height as i32);

		let pixel = self.data.get_pixel(px as u32, self.height - 1 - py as u32).to_rgb();

		Vec3f::new(
			pixel[0] as f64 / 255.0,
			pixel[1] as f64 / 255.0,
			pixel[2] as f64 / 255.0,
		)
	}
}
