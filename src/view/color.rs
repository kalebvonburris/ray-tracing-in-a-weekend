use nalgebra::Vector3;

use crate::{view::interval::Interval, Float, Int};

/// An interval that defines the range of a color channel.
static INTENSITY: Interval = Interval::new(0.000, 0.999);

/// A color - represented as a 3D vector.
///
/// Each component of the vector represents a color channel:
/// - `x`: Red channel.
/// - `y`: Green channel.
/// - `z`: Blue channel.
pub type Color = Vector3<Float>;

/// Write a color to a string in PPM format.
pub fn write_color(string: &mut String, color: Color) {
    let ir = (255.999 * INTENSITY.clamp(color.x)) as Int;
    let ig = (255.999 * INTENSITY.clamp(color.y)) as Int;
    let ib = (255.999 * INTENSITY.clamp(color.z)) as Int;

    string.push_str(&format!("{ir} {ig} {ib}\n"));
}
