use std::process::Command;

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use crate::utils::log::*;

#[derive(Debug, Deserialize, Serialize)]
struct Repo {
    tag_name: String,
}

pub fn self_upgrade() {
    let current_version = env!("CARGO_PKG_VERSION");

    let latest_version_req = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/42.0.2311.135 Safari/537.36 Edge/12.10240")
        .build()
        .unwrap()
        .get("https://api.github.com/repos/imf4ll/opio/releases/latest")
        .send()
        .unwrap()
        .json::<Repo>()
        .unwrap();
        
    let latest_version = latest_version_req.tag_name
        .split("v")
        .collect::<Vec<&str>>()[1];

    if current_version != latest_version {
        wait(&format!("Found a new version: \x1b[1;37mv{latest_version} [ current: v{current_version} ]"));

        Command::new("cargo")
            .arg("install")
            .arg("--git")
            .arg("https://github.com/imf4ll/opio")
            .arg("--force")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();

        println!("");

        success(&format!("Successfully updated to \x1b[1;37mv{latest_version}"));
    
    } else {
        error("Already up to date, aborting.");

    }
}
