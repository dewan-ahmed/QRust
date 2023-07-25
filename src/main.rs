// Write a Rust program that generates a QR code png file from a given URL.
// The program should take one command line arguments - the URL.

use qrcode::QrCode;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let url = &args[1];
    let code = QrCode::new(url).unwrap();
    let image = code.render::<char>().quiet_zone(false).build();
    let path = Path::new("qrcode.png");
    image.save(path).expect("Failed to save image to output path");
}

// $ cargo run https://www.rust-lang.org/