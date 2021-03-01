use std::error::Error;
use std::io::Write;
use std::path::Path;
use std::time::{Duration, Instant};

use clap::{crate_authors, crate_name, crate_version, App, Arg};

use content_aware_resize::energy_function;
use content_aware_resize::Image;

/// Pretty-print a duration.
fn print_duration(duration: Duration) {
    let mut s = duration.as_millis() / 1000;
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

    if h != 0 {
        print!("{}h", h);
    }
    if m != 0 || h != 0 {
        print!("{}m", m);
    }
    print!("{}.{:03}s", s, ms);
}

fn main() -> Result<(), Box<dyn Error>> {
    let arg_matches = App::new(crate_name!())
        .version(crate_version!())
        .about("Resize images while taking their content into account")
        .author(crate_authors!())
        .arg(
            Arg::with_name("INPUT")
                .help("Path of the input image")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("output")
                .help("Path of the output image")
                .short("o")
                .long("output")
        )
        .arg(
            Arg::with_name("width")
                .help("The new width")
                .short("w")
                .long("width")
                .takes_value(true)
                .required_unless("energy")
                .conflicts_with("height"),
        )
        .arg(
            Arg::with_name("height")
                .help("The new height")
                .short("h")
                .long("height")
                .takes_value(true)
                .required_unless("energy")
                .conflicts_with("width"),
        )
        .arg(
            Arg::with_name("energy")
                .help("Dump pixel energy values to an image")
                .short("e")
                .long("energy"),
        )
        .get_matches();

    let mut stdout = std::io::stdout();

    // Load image

    print!("Loading image...");
    stdout.flush()?;

    let start = Instant::now();

    let input_path = Path::new(arg_matches.value_of("INPUT").unwrap());
    let input_file_name = input_path.file_stem().unwrap().to_string_lossy();

    let img = image::open(input_path)?;
    let mut img = Image::from_dynamic_image(img);

    print!(" Finished in ");
    print_duration(start.elapsed());
    println!();

    // Resize horizontally

    if let Some(new_width) = arg_matches.value_of("width") {
        print!("Resizing horizontally...");
        stdout.flush()?;

        let start = Instant::now();

        let new_width = new_width.parse()?;

        img.resize_horizontal(new_width, energy_function::basic);

        print!(" Finished in ");
        print_duration(start.elapsed());
        println!();
    }

    // Resize vertically

    if let Some(new_height) = arg_matches.value_of("height") {
        print!("Resizing vertically...");
        stdout.flush()?;

        let start = Instant::now();

        let new_height = new_height.parse()?;

        img.resize_vertical(new_height, energy_function::basic);

        print!(" Finished in ");
        print_duration(start.elapsed());
        println!();
    }

    // Dump energy

    if arg_matches.is_present("energy") {
        print!("Dumping energy...");
        stdout.flush()?;

        let start = Instant::now();

        let energy_file_name = format!("{}_energy", input_file_name);
        let mut energy_path = input_path.with_file_name(energy_file_name);
        energy_path.set_extension("png");

        img.energy(energy_function::basic)
            .save(energy_path)?;

        print!(" Finished in ");
        print_duration(start.elapsed());
        println!();
    }

    // Write output

    let output_path = if let Some(path) = arg_matches.value_of("output") {
        path.to_string()
    } else {
        let output_file_name =
            format!("{}_{}x{}", input_file_name, img.width(), img.height());
        let mut output_path = input_path.with_file_name(output_file_name);
        output_path.set_extension("png");
        output_path.to_string_lossy().into_owned()
    };

    print!("Saving output...");
    stdout.flush()?;

    let start = Instant::now();

    img.into_image_buffer().save(output_path)?;

    print!(" Finished in ");
    print_duration(start.elapsed());
    println!();

    Ok(())
}
