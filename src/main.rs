use std::env;

use image::{DynamicImage, RgbImage, ColorType};

fn energy(img: &RgbImage, x: u32, y: u32) -> u32 {
    // let (w, h) = img.dimensions();

    // let h1 = if x == 0 { 0 } else { x - 1 };
    // let h2 = (x + 1).min(w - 1);
    // let v1 = if y == 0 { 0 } else { y - 1 };
    // let v2 = (y + 1).min(h - 1);

    // let h1 = &img.get_pixel(h1, y).data;
    // let h2 = &img.get_pixel(h2, y).data;
    // let v1 = &img.get_pixel(x, v1).data;
    // let v2 = &img.get_pixel(x, v2).data;

    // let mut dx = 0;
    // let mut dy = 0;

    // for i in 0..3 {
    //     let dh = (h1[i] as i16) - (h2[i] as i16);
    //     let dv = (v1[i] as i16) - (v2[i] as i16);

    //     dx += (dh * dh) as u32;
    //     dy += (dv * dv) as u32;
    // }

    // return dx + dy;

    return if x % 25 == 0 { 1 } else { 0 };
}

fn main() {
    let file = env::args().nth(1).expect("Expected a file");
    let img = image::open(file).expect("Failed to open image");

    let img_buf = if let DynamicImage::ImageRgb8(buf) = img {
        buf
    } else {
        img.to_rgb()
    };

    let mut cur_min = energy(&img_buf, 0, 0);
    let mut cur_max = cur_min;
    let e_buf: Vec<f32> = img_buf.enumerate_pixels()
        .map(|(x, y, _)| {
            let e = energy(&img_buf, x, y);
            cur_min = cur_min.min(e);
            cur_max = cur_max.max(e);
            e as f32
        })
        .collect();
    
    let range = (cur_max - cur_min) as f32;
    let output: Vec<u8> = e_buf.into_iter()
        .map(|e| (255.0 * (e - (cur_min as f32)) / range) as u8)
        .collect();
    
    image::save_buffer(
        "energy.png", &output,
        img_buf.width(), img_buf.height(),
        ColorType::Gray(8)
    ).unwrap();
}
