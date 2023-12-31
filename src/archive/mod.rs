use std::fs::read_dir;
use std::process::Command;

use crate::utils::packages::*;
use crate::utils::log::{ERROR, error};

use reqwest::get;
use dialoguer::{Select, theme::ColorfulTheme};
use indicatif::{ProgressBar, ProgressStyle};
use futures::StreamExt;

pub async fn get_package(package: &str, package_name: &str, ignore_cache: bool) -> Result<Vec<String>, reqwest::Error> {
    let packages_req = get(package)
        .await?;

    if packages_req.status() != 200 {
        error("Invalid package.");

    }

    let packages_data = packages_req.text().await?;

    let mut packages: Vec<String> = vec![];

    let package_count = packages_data.matches(".pkg.tar.zst\">").count();

    for p in 1..package_count {
        let p = &packages_data
            .split(".pkg.tar.zst\">").collect::<Vec<&str>>()[p]
            .split("<a href=\"").collect::<Vec<&str>>()[0];
        
        let package = &p
            .replace("</a>", "")
            .split(" ").collect::<Vec<&str>>()
            .into_iter()
            .filter(| i | i != &"")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(| i | i.replace("\r", ""))
            .map(| i | i.replace("\n", ""))
            .collect::<Vec<String>>();

        packages.push(format!("{} \x1b[1;34m{}\x1b[m \x1b[1;37m{}\x1b[m", package[0], package[1], package[2..].join(" ")));
    }

    if !ignore_cache {
        for (k, p) in packages.clone().into_iter().enumerate() {
            for c in get_cache(package_name) {
                if p.contains(&c) {
                    packages[k] = format!("{c} \x1b[1;34m[Cache]\x1b[m", );

                }
            }
        }
    }

    Ok(reversed(&packages))
}

fn get_cache(package: &str) -> Vec<String> {
    let mut packages = vec![];

    read_dir("/var/cache/pacman/pkg")
        .expect(&format!("{ERROR} Failed to get cache, are you rooted?"))
        .into_iter()
        .for_each(| i | {
            if format!("{:?}", i.as_ref().expect(&format!("{ERROR} Failed to get cache.")).file_name()).contains(&package) {
                packages.push(format!("{:?}", i.expect(&format!("{ERROR} Failed to get cache.")).file_name()).replace("\"", ""));

            }
        });

    packages
}

pub fn select_package(package: String, packages: &Vec<String>) -> usize {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt(&format!("Choose package{}", get_current_version(&package)))
        .default(0)
        .max_length(15)
        .items(&packages)
        .interact()
        .unwrap()
}

pub async fn download_package(package_path: String, package_name: &str, package: String, no_confirm: bool) -> Result<(), reqwest::Error> {
    let file = tokio::fs::File::create(format!("{package_path}/{package_name}"))
        .await
        .expect(&format!("{ERROR} Failed to create package file, are you rooted?"));

    let data = get(&package)
        .await?;

    let total_size = data
        .content_length()
        .expect(&format!("{ERROR} Failed to get package content length."));

    let pb = ProgressBar::new(total_size);

    pb.set_style(ProgressStyle::default_bar()
        .template(&format!("{{spinner:.green}} {{bar:30.blue/white}} {{bytes}}/{{total_bytes}} ({{eta}})"))
        .unwrap()
        .progress_chars("❚."));

    println!("\n\x1b[1;34m✔ Downloading:\x1b[m {package_name}...\n");
    
    tokio::task::spawn({
        let mut file = file.try_clone().await.unwrap();
        let mut bytes_stream = data.bytes_stream();

        async move {
            while let Some(stream) = bytes_stream.next().await {
                tokio::io::copy(&mut stream.unwrap().as_ref(), &mut file)
                    .await
                    .expect(&format!("{ERROR} Failed to write package, are you rooted?"));
            }
        }
    });

    while get_file_size(&file).await < total_size {
        let file_size = get_file_size(&file).await;

        pb.set_position(file_size);
    }

    pb.finish();

    install_package(format!("{}/{}", package_path, package_name), no_confirm);

    Ok(())
}

pub fn install_package(package: String, noconfirm: bool) {
    if noconfirm {
        Command::new("sudo")
            .arg("pacman")
            .arg("-U")
            .arg(&package)
            .arg("--noconfirm")
            .spawn()
            .expect(&format!("{ERROR} Failed to run pacman as sudo, are you rooted?"))
            .wait()
            .expect(&format!("{ERROR} Failed to run pacman as sudo, are you rooted?"));
        
    } else {
        Command::new("sudo")
            .arg("pacman")
            .arg("-U")
            .arg(&package)
            .spawn()
            .expect(&format!("{ERROR} Failed to run pacman as sudo, are you rooted?"))
            .wait()
            .expect(&format!("{ERROR} Failed to run pacman as sudo, are you rooted?"));
    }
}
