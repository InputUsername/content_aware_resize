use std::env;

use image::{DynamicImage, RgbImage};

fn energy(img: &RgbImage, x: u32, y: u32) -> u32 {
    let (w, h) = img.dimensions();

    let h1 = (x - 1).max(0);
    let h2 = (x + 1).min(w - 1);
    let v1 = (y - 1).max(0);
    let v2 = (y + 1).min(h - 1);

    let h1 = &img.get_pixel((h1), y).data;
    let h2 = &img.get_pixel(h2, y).data;
    let v1 = &img.get_pixel(x, v1).data;
    let v2 = &img.get_pixel(x, v2).data;

    let mut dx = 0;
    let mut dy = 0;

    for i in 0..3 {
        let dh = u32::from(h1[i]-h2[i]);
        let dv = u32::from(v1[i]-v2[i]);
        dx += dh;
        dy += dv;
    }

    return dx + dy;
}

fn main() {
    let file = env::args().nth(1).expect("Expected a file");
    let img = image::open(file).expect("Failed to open image");

    let img_buf = if let DynamicImage::ImageRgb8(buf) = img {
        buf
    } else {
        img.to_rgb()
    };
}
