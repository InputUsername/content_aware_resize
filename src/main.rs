use std::env;

fn main() {
    let file = env::args().nth(1)
        .expect("Expected a file");
    let img = image::open(file).expect("Failed to open image");
}
