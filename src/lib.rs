//! A simple ray tracer written in Rust.

#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::module_name_repetitions,
    clippy::module_inception
)]

use nalgebra::Point3;
use objects::Sphere;

pub mod objects;
pub mod ppm;
pub mod view;
pub mod world;

// This allows us to specify what types we want to use for our Float and Int types.
// This is useful for when we want to switch between f32 and f64 for our Float type
// without reformatting the entire codebase - same for u32 and u64 for our Int type.
/// The floating point type used - can be either `f32` or `f64`.
pub type Float = f32;
/// The integer type used - can be either `u32` or `u64`.
pub type Int = u32;

/// Generates a `Sphere` with given configuration.
#[must_use]
pub fn create_sphere(center: Point3<Float>, radius: Float) -> Box<Sphere> {
    Box::new(Sphere::new(center, radius))
}
