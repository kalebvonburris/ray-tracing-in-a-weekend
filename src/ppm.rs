/// Writes the given image data to a PPM file.
///
/// # Panics
///
/// Panics if unable to write the file.
pub fn write_image_to_ppm(image: &str) {
    std::fs::write("image.ppm", image).expect("Unable to write file");
}
