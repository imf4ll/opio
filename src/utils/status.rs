use std::io::Write;

use reqwest::blocking::get;

pub fn get_status() {
    get_url("https://archive.archlinux.org");
    get_url("https://aur.archlinux.org");
}

fn get_url(url: &str) {
    print!("\x1b[1;34mTesting\x1b[m {url}: ");
    
    std::io::stdout().flush().unwrap();
    
    match get(url) {
        Ok(d) => {
            if d.status().is_success() {
                println!("\x1b[1;34m✔\x1b[m");
    
            } else {
                println!("\x1b[1;31m✕\x1b[m");

            }
        },
        Err(_) => println!("\x1b[1;31m✕\x1b[m"),
    }
}
