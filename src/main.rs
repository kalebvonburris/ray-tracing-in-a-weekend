use nalgebra::Point3;
use ray_tracing_in_a_weekend::{
    create_sphere,
    objects::hittable::Hittable,
    ppm::write_image_to_ppm,
    view::write_color,
    view::{Camera, Ray},
    world::World,
    Float,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub fn main() {
    // Camera
    let camera = Camera::default();

    // Image
    let mut image = format!("P3\n{} {}\n255\n", camera.image_width, camera.image_height);

    // Delta Vectors
    let pixel_delta_vertical = camera.horizontal / camera.image_width as Float;
    let pixel_delta_horizontal = camera.vertical / camera.image_height as Float;

    // Exact location of the first pixel
    let pixel_00_loc =
        camera.upper_left_corner + (pixel_delta_horizontal + pixel_delta_vertical) / 2.0;

    // Objects
    let objects: Vec<Box<dyn Hittable>> = vec![
        create_sphere(Point3::new(0.0, 0.0, -1.0), 0.5),
        create_sphere(Point3::new(0.0, -100.5, -1.0), 100.0),
    ];

    let world = World::new(objects);

    // Render
    let rest_of_image = (0..camera.image_height)
        .into_par_iter()
        //.progress_count(camera.image_height as u64)
        .map(|j| {
            // 13 here because the the max length of a message is 13.
            //   "255 255 255\n".len() = 13 characters
            let mut local_chunk = String::with_capacity(13 * camera.image_width as usize);
            for i in 0..camera.image_width {
                let pixel_center = pixel_00_loc
                    + (j as Float * pixel_delta_horizontal)
                    + (i as Float * pixel_delta_vertical);

                let ray_dir = pixel_center - camera.origin;

                let ray = Ray::new(camera.origin, ray_dir);

                let color = ray.color(&world);

                write_color(&mut local_chunk, color);
            }
            local_chunk
        })
        .collect::<Vec<String>>()
        .join("");

    image.push_str(&rest_of_image);

    write_image_to_ppm(&image);
}
