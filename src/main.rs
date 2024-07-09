use indicatif::ParallelProgressIterator;
use nalgebra::Point3;
use ray_tracing_in_a_weekend::{
    create_sphere,
    objects::hittable::Hittable,
    ppm::write_image_to_ppm,
    view::{interval::Interval, write_color, Camera, Color},
    world::World,
    Float,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub fn main() {
    // Camera
    let camera = Camera::new(1920, 500, 500);

    // Image
    let mut image = format!("P3\n{} {}\n255\n", camera.image_width, camera.image_height);

    // Objects
    let objects: Vec<Box<dyn Hittable>> = vec![
        create_sphere(Point3::new(0.0, 0.0, -1.0), 0.5),
        create_sphere(Point3::new(0.0, -100.5, -1.0), 100.0),
    ];

    let world = World::new(objects);

    // Render
    let rest_of_image = (0..camera.image_height)
        .into_par_iter()
        .progress_count(camera.image_height as u64)
        .map(|j| {
            // 13 here because the the max length of a message is 13.
            //   "255 255 255\n".len() = 13 characters
            let mut local_chunk = String::with_capacity(13 * camera.image_width as usize);
            for i in 0..camera.image_width {
                let mut color = Color::default();
                for _ in 0..camera.samples_per_pixel {
                    let ray = camera.get_ray(i, j);
                    color += ray.color(
                        &world,
                        Interval::new(0.0, Float::INFINITY),
                        camera.max_depth,
                    );
                }
                write_color(
                    &mut local_chunk,
                    color * (1.0 / camera.samples_per_pixel as Float),
                );
            }
            local_chunk
        })
        .collect::<Vec<String>>()
        .join("");

    image.push_str(&rest_of_image);

    write_image_to_ppm(&image);
}
