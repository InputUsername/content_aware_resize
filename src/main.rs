use std::env;
use std::io;

use image::{DynamicImage, ColorType};

fn energy(img: &[&[u8]], w: usize, h: usize, x: usize, y: usize) -> u32 {
    let h1 = x.saturating_sub(1);
    let h2 = (x + 1).min(w - 1);
    let v1 = y.saturating_sub(1);
    let v2 = (y + 1).min(h - 1);

    let ph1 = &img[y][3*h1..3*h1+3];
    let ph2 = &img[y][3*h2..3*h2+3];
    let pv1 = &img[v1][3*x..3*x+3];
    let pv2 = &img[v2][3*x..3*x+3];

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

fn dump_energy_image(img: &[&[u8]], w: usize, h: usize) -> io::Result<()> {
    let mut min = energy(img, w, h, 0, 0);
    let mut max = min;

    let energies: Vec<f32> =
        (0..w*h).map(|i| (i % w, i / w))
        .map(|(x, y)| {
            let e = energy(img, w, h, x, y);
            min = u32::min(min, e);
            max = u32::max(max, e);
            e as f32
        })
        .collect();
    
    let min = min as f32;
    let max = max as f32;

    let output: Vec<u8> = energies.into_iter()
        .map(|e| (255.0 * (e - min) / (max - min)) as u8)
        .collect();

    image::save_buffer("images/energy.png", &output, w as u32, h as u32, ColorType::Gray(8))
}

fn main() {
    let file = env::args().nth(1).expect("Expected a file");
    let img = image::open(file).expect("Failed to open image");
    let img = if let DynamicImage::ImageRgb8(buf) = img {
        buf
    } else {
        img.to_rgb()
    };

    let w = img.width() as usize;
    let h = img.height() as usize;
    let raw_img = img.into_raw();
    let view: Vec<&[u8]> = raw_img.chunks_exact(3 * w).collect();

    dump_energy_image(&view, w, h).unwrap();
}
