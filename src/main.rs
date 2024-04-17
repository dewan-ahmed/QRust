// Write a Rust program that generates a QR code png file from a given URL.
// The program should take one command line arguments - the URL.

use qrcode::QrCode;
use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    let url = &args[1];
    let code = QrCode::new(url).unwrap();
    let image = code.render::<char>().quiet_zone(false).build();
    let path = "qrcode.png";
    let mut file = File::create(path).unwrap_or_else(|e| panic!("Failed to create file: {}", e));
    file.write_all(image.as_bytes()).unwrap_or_else(|e| panic!("Failed to write image data: {}", e));
}

// $ cargo run https://www.rust-lang.org/
