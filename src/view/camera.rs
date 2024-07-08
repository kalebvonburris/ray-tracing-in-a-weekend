use nalgebra::{Point3, Vector3};

use crate::{Float, Int};

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
}

impl Default for Camera {
    fn default() -> Self {
        let image_width = 2560;
        let aspect_ratio = 16.0 / 9.0;

        let image_height = (((image_width as Float) / aspect_ratio) as Int).max(1);

        let viewport_height = 2.0;
        let viewport_width = viewport_height * ((image_width as Float) / (image_height as Float));

        let focal_length = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
        let vertical = Vector3::new(0.0, -viewport_height, 0.0);
        let upper_left_corner =
            origin - (horizontal / 2.0) - (vertical / 2.0) - Vector3::new(0.0, 0.0, focal_length);

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
        }
    }
}
