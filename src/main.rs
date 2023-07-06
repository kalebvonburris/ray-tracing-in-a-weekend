#![allow(dead_code)]
#![allow(unused_imports)]

// Local libraries
mod vec;
use std::f64::INFINITY;

use rand::Rng;
use vec::{Color, Point3, Vec3};
mod ray;
use ray::Ray;
mod objects;
use objects::*;
mod camera;
use camera::*;

// Extrernal libraries
use indicatif::ProgressIterator;

fn ray_color(ray: &Ray, world: &HittableList, depth: u64) -> Color {
    if depth == 0 { return Color::new(0.0, 0.0, 0.0) }

    if let Some(record) =  world.hit(ray, 0.001, INFINITY) {
        let target = record.p + record.normal + Vec3::random_in_unit_sphere().normalized();
        let ray = Ray::new(record.p, target - record.p);
        0.5 * ray_color(&ray, world, depth - 1)
    }
    else {
        let unit_direction: Vec3 = ray.direction().normalized();
        let t: f64 = 0.5 * (unit_direction.y() + 1.0);
        ((1.0 - t) * Color::new(1.0, 1.0, 1.0)) + (t * Color::new(0.5, 0.7, 1.0))
    }
}

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u64 = 1920;
    let image_height: u64 = ((image_width as f64) / aspect_ratio) as u64;
    let samples_per_pixel: u64 = 100;
    let max_depth: u64 = 25;

    // World

    let mut world: HittableList = HittableList::new(vec![]);

    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));

    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera

    let camera: Camera = Camera::new();

    // PPM Headers.

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    let mut rng = rand::thread_rng();

    for h in (0..image_height as u32).rev().progress() {
        for w in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let random_u: f64 = rng.gen();
                let random_v: f64 = rng.gen();

                let u = (w as f64 + random_u) / ((image_width - 1) as f64);
                let v = (h as f64 + random_v) / ((image_height - 1) as f64);

                let r: Ray = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            
            println!("{}", pixel_color.format_color(samples_per_pixel));
        }
    }
}
