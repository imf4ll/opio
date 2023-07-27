use std::process::Command;
use std::env::set_current_dir;
use std::fs::{remove_dir_all, remove_file};

use futures::StreamExt;
use reqwest::get;
use dialoguer::{Select, theme::ColorfulTheme};

use crate::utils::{log::*, pkgbuild::*};

pub async fn get_package(package_name: &str) -> Result<(String, String, String), reqwest::Error> {
    let package_req = get(format!("https://aur.archlinux.org/packages/{package_name}"))
        .await?;

    let status = package_req.status();

    if status == 404 {
        error("Invalid package, are you sure about the name?");

    }

    let data = package_req.text().await?;

    let package = &data
        .split("Package Details: ")
        .collect::<Vec<&str>>()[1]
        .split("</h2>")
        .collect::<Vec<&str>>()[0]
        .split(" ")
        .collect::<Vec<&str>>();

    let git_url = data
        .split("<a class=\"copy\" href=\"")
        .collect::<Vec<&str>>()[1]
        .split("\"")
        .collect::<Vec<&str>>()[0];

    Ok((package[0].to_string(), package[1].to_string(), git_url.to_string()))
}

pub async fn get_downgrade(package_name: &str) -> Result<Vec<String>, reqwest::Error> {
    let package_req = get(format!("https://aur.archlinux.org/cgit/aur.git/log/?h={package_name}"))
        .await?;

    let status = package_req.status();

    if status == 404 {
        error("Invalid package, are you sure about the name?");

    }

    let data = package_req.text().await?;

    let packages_body = data
        .split("<table class='list nowrap'>")
        .collect::<Vec<&str>>()[1]
        .split("</table>")
        .collect::<Vec<&str>>()[0];

    let mut packages = vec![];

    for i in 1..packages_body.matches("<tr>").count() + 1 {
        let mut package_str = "".to_string();

        let package = packages_body
            .split("<tr>")
            .collect::<Vec<&str>>()[i]
            .split("</tr>")
            .collect::<Vec<&str>>()[0];

        for d in 1..package.matches("<td>").count() + 1 {
            let detail = package
                .split("<td>")
                .collect::<Vec<&str>>()[d]
                .split("</td>")
                .collect::<Vec<&str>>()[0];

            if detail.contains("</a>") {
                package_str.push_str(&format!("\x1b[1;34m[Commit message: {}][.]\x1b[m", detail
                    .split(">")
                    .collect::<Vec<&str>>()[1]
                    .split("<")
                    .collect::<Vec<&str>>()[0]));
            
            } else if detail.contains("</span>") {
                package_str.push_str(&format!("{}[.]", detail
                    .split(">")
                    .collect::<Vec<&str>>()[1]
                    .split("<")
                    .collect::<Vec<&str>>()[0]));
            
            } else {
                package_str.push_str(&format!("\x1b[1;37mby: {detail}\x1b[m"));

            }
        }

        package_str.push_str(&format!("[.]{}", package
            .split("<td><a href=\'")
            .collect::<Vec<&str>>()[1]
            .split("\'>")
            .collect::<Vec<&str>>()[0]));

        packages.push(package_str);
    }

    Ok(packages)
}

pub fn install_package(package_name: String, git_url: String, file_path: String, keep: bool, pkgbuild: bool, noconfirm: bool) {
    Command::new("git")
        .arg("clone")
        .arg(git_url)
        .arg(format!("{file_path}/{package_name}"))
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    set_current_dir(format!("{file_path}/{package_name}")).unwrap();

    if pkgbuild { read_pkgbuild("./PKGBUILD"); }

    if noconfirm {
        Command::new("makepkg")
            .arg("-si")
            .arg("--noconfirm")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();

    } else {
        Command::new("makepkg")
            .arg("-si")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }

    if !keep {
        remove_dir_all(format!("{file_path}/{package_name}")).expect(&format!("\n{ERROR} Failed to clean package files."));

    }
}

pub async fn search_package(query: &String) -> Result<Vec<String>, reqwest::Error> {
    let mut packages: Vec<String> = vec![];

    let data = get(format!("https://aur.archlinux.org/packages?K={query}&SeB=n"))
        .await?
        .text()
        .await?;

    let mut _packages_body = "";

    if data.contains("<tbody>") {
        _packages_body = data
            .split("<tbody>")
            .collect::<Vec<&str>>()[1]
            .split("</tbody>")
            .collect::<Vec<&str>>()[0];
    
    } else {
        return Ok(packages);

    }
    
    for p in 1.._packages_body.matches("<tr>").count() + 1 {
        let package = _packages_body
            .split("<tr>")
            .collect::<Vec<&str>>()[p]
            .split("</tr>")
            .collect::<Vec<&str>>()[0]
            .trim();

        let mut name = "";
        let mut version = "";
        let mut date = "";
        let mut outdated = false;

        for d in 1..package.matches("<td").count() + 1 {
            let detail = package
                .split("<td")
                .collect::<Vec<&str>>()[d]
                .split("</td>")
                .collect::<Vec<&str>>()[0];

            if d == 2 {
                version = detail
                    .split(">")
                    .collect::<Vec<&str>>()[1]
                    .split("</td")
                    .collect::<Vec<&str>>()[0];
            
            } else if d == package.matches("<td").count() {
                date = detail
                    .split(">")
                    .collect::<Vec<&str>>()[1]
                    .split("</td")
                    .collect::<Vec<&str>>()[0];
            }

            if detail.contains("/packages/") {
                name = detail
                    .split("/packages/")
                    .collect::<Vec<&str>>()[1]
                    .split("\"")
                    .collect::<Vec<&str>>()[0];
            
            } else if detail.contains("flagged") {
                outdated = true;

                if detail.contains("UTC") {
                    date = detail
                        .split("\">")
                        .collect::<Vec<&str>>()[1];
                
                } else {
                    version = detail
                        .split("\">")
                        .collect::<Vec<&str>>()[1];
                }
            }
        }

        if outdated {
            packages.push(format!("{name} \x1b[1;34m[{version}] \x1b[1;37m{date} \x1b[1;31m[FLAGGED OUTDATED]\x1b[m"));
        
        } else {
            packages.push(format!("{name} \x1b[1;34m[{version}] \x1b[1;37m{date}\x1b[m"));
        
        }
    }

    Ok(packages)
}

pub fn select_package(packages: &Vec<String>) -> usize {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select package:")
        .default(0)
        .items(&packages)
        .max_length(15)
        .interact()
        .unwrap()
}

pub async fn install_downgrade(commit_url: &str, file_path: String, keep: bool, pkgbuild: bool, noconfirm: bool) -> Result<(), reqwest::Error> {
    let commit_req = get(format!("https://aur.archlinux.org{}", commit_url.replace("amp;", "")))
        .await?
        .text()
        .await?;

    let download_url = format!("https://aur.archlinux.org{}", commit_req
        .split("<th>download</th><td colspan='2' class='oid'><a href='")
        .collect::<Vec<&str>>()[1]
        .split("'")
        .collect::<Vec<&str>>()[0]);

    let commit_name = download_url.split("/snapshot/").collect::<Vec<&str>>()[1].to_string();

    let mut file = tokio::fs::File::create(format!("{file_path}/{commit_name}"))
        .await
        .expect(&format!("{ERROR} Failed to create commit file, are you rooted?"));
    
    println!("\n\x1b[1;34m✔ Downloading:\x1b[m {commit_name}...\n");
    
    let mut data = get(download_url)
        .await?
        .bytes_stream();

    while let Some(stream) = data.next().await {
        tokio::io::copy(&mut stream?.as_ref(), &mut file)
            .await
            .expect(&format!("{ERROR} Failed to write package, are you rooted?"));
    }

    install_commit_package(commit_name, file_path, keep, pkgbuild, noconfirm);

    Ok(())
}

pub fn install_commit_package(commit_name: String, file_path: String, keep: bool, pkgbuild: bool, noconfirm: bool) {
    let package_name = commit_name.split(".tar.gz").collect::<Vec<&str>>()[0];

    set_current_dir(&file_path).unwrap();
    
    Command::new("tar")
        .arg("-xvf")
        .arg(&commit_name)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    set_current_dir(format!("{file_path}/{package_name}")).unwrap();
    
    if pkgbuild { read_pkgbuild("./PKGBUILD"); }
    
    if noconfirm {
        Command::new("makepkg")
            .arg("-si")
            .arg("--noconfirm")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();

    } else {
        Command::new("makepkg")
            .arg("-si")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();

    }

    set_current_dir("../").unwrap();
      
    remove_file(format!("{file_path}/{commit_name}")).unwrap();

    if !keep {
        remove_dir_all(format!("{file_path}/{package_name}")).unwrap();

    }
}
