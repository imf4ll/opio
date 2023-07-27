use std::io::Write;

use reqwest::get;

pub async fn get_status() -> Result<(), reqwest::Error> {
    get_url("https://archive.archlinux.org").await?;
    get_url("https://aur.archlinux.org").await?;
    
    Ok(())
}

async fn get_url(url: &str) -> Result<(), reqwest::Error> {
    print!("\x1b[1;34mTesting\x1b[m {url}: ");
    
    std::io::stdout().flush().unwrap();
    
    match get(url).await {
        Ok(d) => {
            if d.status().is_success() {
                println!("\x1b[1;34m✔\x1b[m");
    
            } else {
                println!("\x1b[1;31m✕\x1b[m");

            }
        },
        Err(_) => println!("\x1b[1;31m✕\x1b[m"),
    };

    Ok(())
}
