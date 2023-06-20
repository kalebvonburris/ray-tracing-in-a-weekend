use std::io::{stderr, Write};

fn main() {
    // Consts for image dimensions.
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = 256;

    // PPM Headers.
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for h in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:3}", IMAGE_HEIGHT - h - 1);
        stderr().flush().unwrap();

        for w in 0..IMAGE_WIDTH {
            let r = (w as f64) / ((IMAGE_WIDTH - 1) as f64);
            let g = (h as f64) / ((IMAGE_HEIGHT - 1) as f64);
            let b = 0.25;

            let ir = (255.999 * r) as u64;
            let ig = (255.999 * g) as u64;
            let ib = (255.999 * b) as u64;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
