use std::process::exit;

pub static ERROR: &str = "\x1b[1;31m✕\x1b[m";

pub fn error(message: &str) {
    println!("\x1b[1;31m✕\x1b[m {message}");

    exit(2);
}
