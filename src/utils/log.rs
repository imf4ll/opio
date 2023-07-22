use std::process::exit;
use std::io::{Write, stdin, stdout};

pub static ERROR: &str = "\x1b[1;31m✕\x1b[m";

pub fn error(message: &str) {
    println!("\x1b[1;31m✕\x1b[m {message}");

    exit(2);
}

pub fn success(message: &str) {
    println!("\x1b[1;34m✔ {message}\x1b[m\n");
}

pub fn warning(message: &str) {
    println!("\x1b[1;33m⚠ {message}\x1b[m\n");
}

pub fn prompt(message: &str) {
    let mut confirm = String::new();

    print!("\n\x1b[1;33m❓{message}\x1b[m ");

    stdout().flush().unwrap();

    stdin().read_line(&mut confirm).unwrap();

    if confirm.trim().to_lowercase() != "y" {
        println!("\nAborted by user.");

        exit(2);
    }

    println!("");
}
