use nalgebra::Point3;

use crate::{
    view::{interval::Interval, ray::Ray},
    Float,
};

use super::hittable::{HitRecord, Hittable};

pub struct Sphere {
    pub center: Point3<Float>,
    pub radius: Float,
}

impl Sphere {
    #[must_use]
    pub const fn new(center: Point3<Float>, radius: Float) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    #[must_use]
    fn hit(&self, ray: Ray, interval: &Interval) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.magnitude_squared();
        let half_b = oc.dot(&ray.direction);
        let c = self.radius.mul_add(-self.radius, oc.magnitude_squared());
        let discriminant = half_b.mul_add(half_b, -(a * c));

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if !interval.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !interval.surrounds(root) {
                return None;
            }
        }

        let point = ray.at(root);
        let normal = (point - self.center) / self.radius;

        let mut record = HitRecord::new(point, normal, root);

        let outward_normal = (point - self.center) / self.radius;

        record.set_face_normal(&ray, outward_normal);

        Some(record)
    }
}
