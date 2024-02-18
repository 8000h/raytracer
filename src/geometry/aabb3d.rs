use crate::geometry::{Interval, Ray, Vec3f};

use std::mem::swap;

pub struct Aabb3d {
	bounds: [Interval; 3],
}

impl Aabb3d {
	pub fn from_intervals(x_bound: Interval, y_bound: Interval, z_bound: Interval) -> Aabb3d {
		Aabb3d {
			bounds: [x_bound, y_bound, z_bound],
		}
	}

	pub fn from_corners(corner0: Vec3f, corner1: Vec3f) -> Aabb3d {
		let ix = Interval::new(
			f64::min(corner0.x, corner1.x),
			f64::max(corner0.x, corner1.x),
		);
		let iy = Interval::new(
			f64::min(corner0.y, corner1.y),
			f64::max(corner0.y, corner1.y),
		);
		let iz = Interval::new(
			f64::min(corner0.z, corner1.z),
			f64::max(corner0.z, corner1.z),
		);

		Aabb3d::from_intervals(ix, iy, iz)
	}

	pub fn from_bounds(bounds0: &Aabb3d, bounds1: &Aabb3d) -> Aabb3d {
		Aabb3d {
			bounds: [
				Interval::from_intervals(&bounds0.bounds[0], &bounds1.bounds[0]),
				Interval::from_intervals(&bounds0.bounds[1], &bounds1.bounds[1]),
				Interval::from_intervals(&bounds0.bounds[2], &bounds1.bounds[2]),
			],
		}
	}

	pub fn hit(&self, ray: &Ray, interval: &Interval) -> bool {
		let mut imin: f64 = interval.min;
		let mut imax: f64 = interval.max;

		for a in 0..3 {
			let inv_d = 1.0 / ray.direction[a];
			let orig = ray.origin[a];

			let mut t0 = (self.bounds[a].min - orig) * inv_d;
			let mut t1 = (self.bounds[a].max - orig) * inv_d;

			if inv_d < 0.0 {
				swap(&mut t0, &mut t1);
			}

			if t0 > imin {
				imin = t0;
			}

			if t1 < imax {
				imax = t1;
			}

			if imax <= imin {
				return false;
			}
		}

		return true;
	}

	pub fn pad(bounds: &Aabb3d) -> Aabb3d {
		const DELTA: f64 = 0.00001;
		let ix = if bounds.bounds[0].size() >= DELTA {
			bounds.bounds[0]
		} else {
			Interval::expand(&bounds.bounds[0], DELTA)
		};
		let iy = if bounds.bounds[1].size() >= DELTA {
			bounds.bounds[1]
		} else {
			Interval::expand(&bounds.bounds[1], DELTA)
		};
		let iz = if bounds.bounds[2].size() >= DELTA {
			bounds.bounds[2]
		} else {
			Interval::expand(&bounds.bounds[2], DELTA)
		};

		Aabb3d {
			bounds: [ix, iy, iz],
		}
	}

	pub fn lt(lhs: &Aabb3d, rhs: &Aabb3d, axis: usize) -> bool {
		lhs.bounds[axis].min < rhs.bounds[axis].min
	}
}

impl Default for Aabb3d {
	fn default() -> Self {
		Aabb3d {
			bounds: [
				Interval::new(0.0, 0.0),
				Interval::new(0.0, 0.0),
				Interval::new(0.0, 0.0),
			],
		}
	}
}
