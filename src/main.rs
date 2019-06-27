use std::env;

use image::DynamicImage;

fn main() {
    let file = env::args().nth(1)
        .expect("Expected a file");
    let img = image::open(file).expect("Failed to open image");

    let img_buf = if let DynamicImage::ImageRgb8(buf) = img {
        buf
    } else {
        img.to_rgb()
    };
}
