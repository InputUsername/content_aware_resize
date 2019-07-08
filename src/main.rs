mod energy_function;

use std::env;
use std::io;

use image::{DynamicImage, ColorType};

struct Energy {
    value: u32,
    backptr: usize
}

fn find_min_energy_seam<F>(img: &[&[u8]], w: usize, h: usize, energy_function: F) -> Vec<usize>
    where F: Fn(&[&[u8]], usize, usize, usize, usize) -> u32
{
    let mut energy_buf: Vec<Energy> = Vec::with_capacity(w * h);
    for x in 0..w {
        energy_buf.push(Energy {
            value: energy_function(img, w, h, x, 0),
            backptr: 0
        });
    }

    for y in 1..h {
        for x in 0..w {
            let prev_row_start = (y - 1) * w;
            let lo = x.saturating_sub(1);
            let hi = usize::min(x + 1, w - 1) + 1;

            let (prev_min, prev_min_x) = energy_buf[prev_row_start + lo..prev_row_start + hi].iter()
                .zip(lo..hi)
                .min_by_key(|(e, _)| e.value)
                .unwrap();
            let prev_min_value = prev_min.value;

            energy_buf.push(Energy {
                value: prev_min_value + energy_function(img, w, h, x, y),
                backptr: prev_min_x
            });
        }
    }

    let mut seam = Vec::with_capacity(h);

    let last_row_start = (h - 1)*w;
    let (mut seam_x_pos, _) = energy_buf[last_row_start..].iter()
        .enumerate()
        .min_by_key(|(_, e)| e.value)
        .unwrap();

    for y in (0..h).rev() {
        seam.push(seam_x_pos);
        seam_x_pos = energy_buf[y * w + seam_x_pos].backptr;
    }

    seam
}

fn dump_energy_image(img: &[&[u8]], w: usize, h: usize) -> io::Result<()> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min_energy_seam() {
        const ENERGIES: &[&[u32]] = &[
            &[9, 9, 0, 9, 9],
            &[9, 1, 9, 8, 9],
            &[9, 9, 9, 9, 0],
            &[9, 9, 9, 0, 9]
        ];

        let w = ENERGIES[0].len();
        let h = ENERGIES.len();

        let seam = find_min_energy_seam(&[&[]], w, h, |_img, _w, _h, x, y| ENERGIES[y][x]);

        assert_eq!(seam, [3, 4, 3, 2]);
    }
}