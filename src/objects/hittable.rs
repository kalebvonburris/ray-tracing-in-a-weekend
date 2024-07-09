use nalgebra::{Point3, Vector3};

use crate::{
    view::{interval::Interval, Ray},
    Float,
};

pub trait Hittable: Sync + Send {
    /// Check if a ray hit an object.
    ///
    /// # Arguments
    /// - `ray`: The ray to check for a hit.
    /// - `t_min`: The minimum time for a hit.
    /// - `t_max`: The maximum time for a hit.
    fn hit(&self, ray: Ray, interval: &Interval) -> Option<HitRecord>;
}

/// The record of a `Ray` hitting an object.
pub struct HitRecord {
    /// The point where the ray hit the object.
    pub point: Point3<Float>,
    /// The normal of the object at the point where the ray hit the object.
    pub normal: Vector3<Float>,
    /// The time at which the ray hit the object.
    pub time: Float,
    /// Whether the `Ray` hit a part of the object facing it.
    pub front_face: bool,
}

impl HitRecord {
    #[must_use]
    pub const fn new(point: Point3<Float>, normal: Vector3<Float>, time: Float) -> Self {
        Self {
            point,
            normal,
            time,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3<Float>) {
        self.front_face = if ray.direction.dot(&outward_normal) > 0.0 {
            self.normal = -outward_normal;
            true
        } else {
            false
        };
    }
}
