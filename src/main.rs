mod energy_function;
mod seam;

use std::io;
use std::path::Path;
use std::time::{Duration, Instant};

use image::{ColorType, DynamicImage};

use clap::{crate_version, App, Arg};

fn dump_energy_image(img: &[u8], w: usize, h: usize) -> io::Result<()> {
    let mut min = energy_function::basic(img, w, h, 0, 0);
    let mut max = min;

    let energies: Vec<f32> = (0..w * h)
        .map(|i| (i % w, i / w))
        .map(|(x, y)| {
            let e = energy_function::basic(img, w, h, x, y);
            min = u32::min(min, e);
            max = u32::max(max, e);
            e as f32
        })
        .collect();

    let min = min as f32;
    let max = max as f32;

    let output: Vec<u8> = energies
        .into_iter()
        .map(|e| (255.0 * (e - min) / (max - min)) as u8)
        .collect();

    image::save_buffer(
        "images/energy.png",
        &output,
        w as u32,
        h as u32,
        ColorType::Gray(8),
    )
}

fn content_aware_resize(img: &mut Vec<u8>, w: usize, h: usize, new_w: usize) {
    assert!(new_w < w);

    let mut energy_buffer = seam::make_energy_buffer(w, h);

    let mut cur_w = w;
    while cur_w > new_w {
        seam::remove_min_energy_seam(img, cur_w, h, energy_function::basic, &mut energy_buffer);

        energy_buffer.clear();

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
    let matches = App::new("content_aware_resize")
        .version(crate_version!())
        .about("Resize images while taking their content into account")
        .author("Koen Bolhuis")
        .arg(
            Arg::with_name("INPUT")
                .help("The input image")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("The output file")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("width")
                .help("The new width of the result image")
                .required(true)
                .short("w")
                .long("width")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("energy")
                .help("Dump an image containing the energy of the input image")
                .short("e")
                .long("energy"),
        )
        .get_matches();

    let input_file = matches.value_of("INPUT").unwrap();
    let input_path = Path::new(input_file);

    let output_file = matches.value_of("OUTPUT").unwrap();
    let output_path = Path::new(output_file);

    let img = match image::open(input_path) {
        Ok(img) => img,
        Err(_err) => {
            eprintln!("Could not open the input image");
            return;
        }
    };
    let img = if let DynamicImage::ImageRgb8(buf) = img {
        buf
    } else {
        img.to_rgb()
    };

    let new_w = matches.value_of("width").unwrap();
    let new_w: usize = match new_w.parse() {
        Ok(val) => val,
        Err(_err) => {
            eprintln!("New width must be an integer");
            return;
        }
    };

    let w = img.width() as usize;
    let h = img.height() as usize;
    let mut img = img.into_raw();

    if matches.is_present("energy") {
        match dump_energy_image(&img, w, h) {
            Ok(_) => (),
            Err(_err) => {
                eprintln!("Failed to dump energy image");
                return;
            }
        }
    }

    let start = Instant::now();

    content_aware_resize(&mut img, w, h, new_w);

    assert_eq!(img.len(), 3 * new_w * h);

    match image::save_buffer(output_path, &img, new_w as u32, h as u32, ColorType::RGB(8)) {
        Ok(_) => (),
        Err(_err) => {
            eprintln!("Could not save output image");
            return;
        }
    }

    print_duration(&start.elapsed());
}
