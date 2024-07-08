use nalgebra::{Point3, Vector3};

use crate::{view::Color, world::World, Float};

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
    pub fn color(&self, world: &World) -> Color {
        for object in world.objects.iter() {
            if let Some(hit_record) = object.hit(*self, 0.0, Float::INFINITY) {
                return 0.5 * (hit_record.normal + Color::new(1.0, 1.0, 1.0));
            }
        }

        let unit_direction = self.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);

        (Color::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t)
    }
}
