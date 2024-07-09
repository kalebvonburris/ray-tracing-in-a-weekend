use nalgebra::{Point3, Vector3};

use crate::{
    get_random_on_hemisphere, objects::hittable::HitRecord, view::Color, world::World, Float, Int,
};

use super::interval::Interval;

/// A `Ray` cast from the `Camera`.
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    /// Where the `Ray` starts from.
    pub origin: Point3<Float>,
    /// The direction the `Ray` is cast in. Can be normalized or not.
    pub direction: Vector3<Float>,
}

impl Ray {
    /// Create a new `Ray`.
    ///
    /// # Arguments
    ///
    /// - `origin`: Where the `Ray` starts from.
    /// - `direction`: The direction the `Ray` is cast in. Can be normalized or not.
    #[must_use]
    pub const fn new(origin: Point3<Float>, direction: Vector3<Float>) -> Self {
        Self { origin, direction }
    }

    /// Get the point at a certain time along the `Ray`.
    ///
    /// # Arguments
    ///
    /// - `t`: The time along the `Ray` to get.
    #[must_use]
    pub fn at(&self, t: Float) -> Point3<Float> {
        self.origin + (t * self.direction)
    }

    /// Get the color of the `Ray`.
    #[must_use]
    pub fn color(&self, world: &World, mut interval: Interval, depth: Int) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut hit: Option<HitRecord> = None;

        for object in world.objects.iter() {
            if let Some(r) = object.hit(*self, &interval) {
                if r.time < interval.max {
                    interval.max = r.time;
                    hit = Some(r);
                }
            }
        }

        if let Some(r) = hit {
            let direction = get_random_on_hemisphere(r.normal);
            return 0.5
                * Self::new(r.point, direction).color(
                    world,
                    Interval::new(0.0, Float::INFINITY),
                    depth - 1,
                );
        }

        let unit_direction = self.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);

        (Color::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t)
    }
}
