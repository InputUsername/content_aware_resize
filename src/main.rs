mod energy_function;
mod seam;

use std::env;
use std::io;
use std::time::{Instant, Duration};

use image::{DynamicImage, ColorType};

fn dump_energy_image(img: &[u8], w: usize, h: usize) -> io::Result<()> {
    let mut min = energy_function::basic(img, w, h, 0, 0);
    let mut max = min;

    let energies: Vec<f32> =
        (0..w*h).map(|i| (i % w, i / w))
        .map(|(x, y)| {
            let e = energy_function::basic(img, w, h, x, y);
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

fn content_aware_resize(img: &mut Vec<u8>, w: usize, h: usize, new_w: usize) {
    assert!(new_w < w);

    let mut cur_w = w;
    while cur_w > new_w {
        seam::remove_min_energy_seam(img, cur_w, h, energy_function::basic);

        cur_w -= 1;
    }

    img.truncate(3 * new_w * h);
}

fn print_duration(duration: &Duration) {
    let ms = duration.as_millis();
    let mut s = ms / 1000;
    let mut m = 0;
    if s > 60 {
        m = s / 60;
        s %= 60;
    }
    let mut h = 0;
    if m > 60 {
        h = m / 60;
        m %= 60;
    }
    let ms = duration.subsec_millis();

    print!("Resizing took ");

    if h != 0 && m != 0 {
        println!("{}h{}m{}.{}s", h, m, s, ms);
    } else if m != 0 {
        println!("{}m{}.{}s", m, s, ms);
    } else {
        println!("{}.{}s", s, ms);
    }
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
    let mut img = img.into_raw();

    let new_w = 896;

    let start = Instant::now();

    content_aware_resize(&mut img, w, h, new_w);

    assert_eq!(img.len(), 3 * new_w * h);

    image::save_buffer("images/arch_small.png", &img, new_w as u32, h as u32, ColorType::RGB(8)).unwrap();

    print_duration(&start.elapsed());
}