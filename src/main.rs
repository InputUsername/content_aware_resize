use std::env;

use image::{DynamicImage, RgbImage, ColorType};

fn energy(img: &RgbImage, x: u32, y: u32) -> u32 {
    let (w, h) = img.dimensions();

    let h1 = x.saturating_sub(1);
    let h2 = (x + 1).min(w - 1);
    let v1 = y.saturating_sub(1);
    let v2 = (y + 1).min(h - 1);

    let ph1 = &img.get_pixel(h1, y).data;
    let ph2 = &img.get_pixel(h2, y).data;
    let pv1 = &img.get_pixel(x, v1).data;
    let pv2 = &img.get_pixel(x, v2).data;

    let mut dx = 0;
    let mut dy = 0;

    for i in 0..3 {
        let ha = ph1[i].max(ph2[i]);
        let hb = ph1[i].min(ph2[i]);
        let va = pv1[i].max(pv2[i]);
        let vb = pv1[i].min(pv2[i]);

        let dh = (ha - hb) as u32;
        let dv = (va - vb) as u32;

        dx += dh * dh;
        dy += dv * dv;
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
