use nalgebra::Vector3;

use crate::{Float, Int};

/// A color - represented as a 3D vector.
///
/// Each component of the vector represents a color channel:
/// - `x`: Red channel.
/// - `y`: Green channel.
/// - `z`: Blue channel.
pub type Color = Vector3<Float>;

/// Write a color to a string in PPM format.
pub fn write_color(string: &mut String, color: Color) {
    let ir = (255.999 * color.x) as Int;
    let ig = (255.999 * color.y) as Int;
    let ib = (255.999 * color.z) as Int;

    string.push_str(&format!("{ir} {ig} {ib}\n"));
}
