use nalgebra::{Point3, Vector3};

use crate::{Float, Int};

use super::Ray;

/// The camera in the application.
pub struct Camera {
    pub aspect_ratio: Float,
    pub viewport_height: Float,
    pub viewport_width: Float,
    pub image_width: Int,
    pub image_height: Int,
    pub focal_length: Float,
    pub origin: Point3<Float>,
    pub horizontal: Vector3<Float>,
    pub vertical: Vector3<Float>,
    pub upper_left_corner: Point3<Float>,
    pub pixel_delta_horizontal: Vector3<Float>,
    pub pixel_delta_vertical: Vector3<Float>,
    pub pixel_00_loc: Point3<Float>,
    pub samples_per_pixel: Int,
    pub max_depth: Int,
}

impl Camera {
    #[must_use]
    pub fn new(image_width: Int, samples_per_pixel: Int, max_depth: Int) -> Self {
        let aspect_ratio = 16.0 / 9.0;

        let image_height = ((Float::from(image_width) / aspect_ratio) as Int).max(1);

        let viewport_height = 2.0;
        let viewport_width =
            viewport_height * (Float::from(image_width) / Float::from(image_height));

        let focal_length = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
        let vertical = Vector3::new(0.0, -viewport_height, 0.0);
        let upper_left_corner =
            origin - (horizontal / 2.0) - (vertical / 2.0) - Vector3::new(0.0, 0.0, focal_length);

        // Delta Vectors
        let pixel_delta_vertical = horizontal / Float::from(image_width);
        let pixel_delta_horizontal = vertical / Float::from(image_height);

        // Exact location of the first pixel
        let pixel_00_loc =
            upper_left_corner + (pixel_delta_horizontal + pixel_delta_vertical) / 2.0;

        Self {
            aspect_ratio,
            viewport_height,
            viewport_width,
            image_width,
            image_height,
            focal_length,
            origin,
            horizontal,
            vertical,
            upper_left_corner,
            pixel_delta_horizontal,
            pixel_delta_vertical,
            pixel_00_loc,
            samples_per_pixel,
            max_depth,
        }
    }

    #[must_use]
    pub fn get_ray(&self, i: Int, j: Int) -> Ray {
        let offset = sample_square();

        let pixel_sample = self.pixel_00_loc
            + ((Float::from(j) + offset.y) * self.pixel_delta_horizontal)
            + ((Float::from(i) + offset.x) * self.pixel_delta_vertical);

        let ray_origin = self.origin;
        let ray_direction = pixel_sample - self.origin;

        Ray::new(ray_origin, ray_direction)
    }
}

#[must_use]
pub fn sample_square() -> Vector3<Float> {
    let x = rand::random::<Float>() - 0.5;
    let y = rand::random::<Float>() - 0.5;

    Vector3::new(x, y, 0.0)
}
