use std::process::exit;

pub static ERROR: &str = "\x1b[1;31m✕\x1b[m";

pub fn error(message: &str) {
    println!("\x1b[1;31m✘\x1b[m {message}");

    exit(2);
}

pub fn wait(message: &str) {
    println!("\x1b[1;34m⟳ {message}\x1b[m\n");
}

pub fn success(message: &str) {
    println!("\x1b[1;34m› {message}\x1b[m\n");
}

pub fn warning(message: &str) {
    println!("\x1b[1;33m⚠ {message}\x1b[m\n");
}
