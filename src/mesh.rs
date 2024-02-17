use std::sync::Arc;

use tobj::*;

use rand::Rng;

use crate::aabb3d::Aabb3d;
use crate::geometry::{HitResult, Hittable, HittableGroup, Interval, Ray};
use crate::material::Material;
use crate::triangle::{Triangle, Uv};
use crate::vec3f::Vec3f;

pub struct Bvh {
	bounds: Aabb3d,
	left: Arc<dyn Hittable>,
	right: Arc<dyn Hittable>,
}

impl Bvh {
	pub fn new(objects: &mut Vec<Arc<dyn Hittable>>) -> Bvh {
		let axis = rand::thread_rng().gen_range(0..=2);
		let span = objects.len();

		let left: Arc<dyn Hittable>;
		let right: Arc<dyn Hittable>;

		if objects.len() == 1 {
			// If leaf node
			left = Arc::clone(&objects[0]);
			right = Arc::clone(&objects[0]);
		} else if objects.len() == 2 {
			if Aabb3d::lt(objects[0].bounds(), objects[1].bounds(), axis) {
				left = Arc::clone(&objects[0]);
				right = Arc::clone(&objects[1]);
			} else {
				left = Arc::clone(&objects[1]);
				right = Arc::clone(&objects[0]);
			}
		} else {
			// Sort the objects on the randomly chosen axis
			objects.sort_by(|a, b| {
				if Aabb3d::lt(&a.bounds(), &b.bounds(), axis) {
					std::cmp::Ordering::Greater
				} else {
					std::cmp::Ordering::Less
				}
			});

			let mid = span / 2;
			let (left_objects, right_objects) = objects.split_at(mid);

			left = Arc::new(Bvh::new(&mut left_objects.to_vec()));
			right = Arc::new(Bvh::new(&mut right_objects.to_vec()));
		}

		Bvh {
			bounds: Aabb3d::from_bounds(&left.bounds(), &right.bounds()),
			left: left,
			right: right,
		}
	}
}

impl Hittable for Bvh {
	fn hit(&self, interval: &Interval, ray: &Ray) -> Option<HitResult> {
		if !self.bounds.hit(ray, interval) {
			return None;
		}

		let hit_left = self.left.hit(interval, ray);
		let hit_right = self.right.hit(
			&Interval::new(
				interval.min,
				if let Some(hit) = hit_left.as_ref() {
					hit.t
				} else {
					interval.max
				},
			),
			ray,
		);

		if hit_left.is_some() {
			hit_left
		} else {
			hit_right
		}
	}

	fn bounds(&self) -> &Aabb3d {
		&self.bounds
	}
}

pub fn load_mesh(path: &str, material: Arc<dyn Material>) -> Bvh {
	println!("Loading {}", path);

	let mut tris: Vec<Arc<dyn Hittable>> = Vec::new();

	let mut options = tobj::LoadOptions::default();

	options.triangulate = true;
	options.single_index = false;

	let (models, materials) = tobj::load_obj(&path, &options).unwrap();

	for (i, m) in models.iter().enumerate() {
		let cmesh = &m.mesh;
		let face_count = cmesh.indices.len() / 3;

		for face in 0..face_count {
			let p0 = cmesh.indices[face * 3] as usize * 3;
			let p1 = cmesh.indices[face * 3 + 1] as usize * 3;
			let p2 = cmesh.indices[face * 3 + 2] as usize * 3;

			let v0 = &cmesh.positions[p0..p0 + 3];
			let v1 = &cmesh.positions[p1..p1 + 3];
			let v2 = &cmesh.positions[p2..p2 + 3];

			let ti0 = cmesh.texcoord_indices[face * 3] as usize * 2;
			let ti1 = cmesh.texcoord_indices[face * 3 + 1] as usize * 2;
			let ti2 = cmesh.texcoord_indices[face * 3 + 2] as usize * 2;

			let t0 = &cmesh.texcoords[ti0..ti0 + 2];
			let t1 = &cmesh.texcoords[ti1..ti1 + 2];
			let t2 = &cmesh.texcoords[ti2..ti2 + 2];

			let temp = Uv::new(0.0, 0.0);

			let tri = Triangle::new(
				Vec3f::new(v0[0] as f64 + 3.5, v0[1] as f64, v0[2] as f64 - 1.5),
				Vec3f::new(v1[0] as f64 + 3.5, v1[1] as f64, v1[2] as f64 - 1.5),
				Vec3f::new(v2[0] as f64 + 3.5, v2[1] as f64, v2[2] as f64 - 1.5),
				//temp,
				//temp,
				//temp,
				Uv::new(t0[0] as f64, t0[1] as f64),
				Uv::new(t1[0] as f64, t1[1] as f64),
				Uv::new(t2[0] as f64, t2[1] as f64),
				Arc::clone(&material),
			);

			tris.push(Arc::new(tri));
		}

		println!(
			"\t{}: {} verts, {} faces",
			m.name,
			m.mesh.positions.len() / 3,
			face_count,
		);
	}

	Bvh::new(&mut tris)
}
