use std::process::Command;
use std::env::set_current_dir;
use std::fs::remove_dir_all;

use reqwest::blocking::get;
use dialoguer::{Select, theme::ColorfulTheme};

use crate::utils::log::*;

pub fn get_package(package_url: String) -> (String, String, String) {
    let package_req = get(package_url)
        .expect(&format!("{ERROR} Invalid package."));

    let status = package_req.status();

    if status == 404 {
        error("Invalid package, are you sure about the name?");

    }

    let data = package_req.text().unwrap();

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

    (package[0].to_string(), package[1].to_string(), git_url.to_string())
}

pub fn install_package(package_name: String, git_url: String) {
    Command::new("git")
        .arg("clone")
        .arg(git_url)
        .arg(format!("/tmp/{package_name}"))
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    set_current_dir(format!("/tmp/{package_name}")).unwrap();

    Command::new("makepkg")
        .arg("-si")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    remove_dir_all(format!("/tmp/{package_name}")).expect(&format!("\n{ERROR} Failed to clean package files."));
}

pub fn search_package(query: &String) -> Vec<String> {
    let mut packages: Vec<String> = vec![];

    let data = get(format!("https://aur.archlinux.org/packages?K={query}&SeB=n"))
        .expect(&format!("{ERROR} Failed on search."))
        .text()
        .unwrap();

    let mut _packages_body = "";

    if data.contains("<tbody>") {
        _packages_body = data
            .split("<tbody>")
            .collect::<Vec<&str>>()[1]
            .split("</tbody>")
            .collect::<Vec<&str>>()[0];
    
    } else {
        return packages;

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
            packages.push(format!("{name} [{version}] [{date}] [FLAGGED OUTDATED]"));
        
        } else {
            packages.push(format!("{name} \x1b[1;34m[{version}]\x1b[m \x1b[1;37m[{date}]\x1b[m"));
        
        }
    }

    packages
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
