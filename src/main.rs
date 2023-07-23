mod utils;
mod downgrade;
mod helper;

use crate::utils::{packages::*, log::*, banner::*, status::*};

use clap::Parser;
use nix::unistd::geteuid;

#[derive(Parser)]
#[command(author, version, about = about(), long_about = None)]
struct Args {
    /// Package name
    #[arg(short, long, value_name = "PACKAGE", default_value = "")]
    package: String,

    /// Final package download path
    #[arg(short, long, value_name = "PATH", default_value = "")]
    file_path: String,

    /// Ignores packages from cache while downgrading 'pacman' packages
    #[arg(short, long, default_value_t = false)]
    ignore_cache: bool,

    /// Turns on downgrade mode
    #[arg(short, long, default_value_t = false)]
    downgrade: bool,

    /// Turns on AUR helper mode
    #[arg(short, long, default_value_t = false)]
    aur: bool,

    /// Search for a package in AUR
    #[arg(short, long, default_value_t = false)]
    search: bool,

    /// Check Archive and AUR status
    #[arg(long, default_value_t = false)]
    status: bool,

    /// Keep AUR package after installing
    #[arg(short, long, default_value_t = false)]
    keep: bool,
}

fn main() {
    let mut args = Args::parse();

    banner();

    if args.file_path == "" {
        if args.downgrade && !args.aur {
            args.file_path = "/var/cache/pacman/pkg/".to_string();

        } else {
            args.file_path = "/tmp/".to_string();

        }
    }

    if args.status {
        get_status();

    } else if args.package != "" {
        if args.downgrade && !args.aur {
            if !geteuid().is_root() {
                error("Superuser privileges needed.");
         
            }

            let package_subject = args.package.chars().collect::<Vec<char>>()[0];
            let package_url = format!("https://archive.archlinux.org/packages/{}/{}", package_subject, args.package);
            
            let packages = downgrade::get_package(&package_url, &args.package, args.ignore_cache);

            if get_current_version(&args.package) == "" {
                warning("Package not installed!");

            }
            
            success(&format!("\x1b[1;37m{} \x1b[1;34mversions found.", packages.len()));
            
            let package = packages[downgrade::select_package(args.package, &packages)].split(" ").collect::<Vec<&str>>();

            if package[1].contains("Cache") {
                downgrade::install_package(format!("/var/cache/pacman/pkg/{}", package[0]));

            } else {
                downgrade::download_package(args.file_path, &package[0], format!("{package_url}/{}", package[0]));

            }

        } else {
            if args.downgrade {
                let packages = helper::downgrade_package(&args.package);

                success(&format!("Listing last \x1b[1;37m{} \x1b[1;34mversions.", packages.len()));

                if get_current_version(&args.package) == "" {
                    warning("Package not installed!");

                }

                let package = &packages[helper::select_package(&packages
                    .clone()
                    .into_iter()
                    .map(| i | i.split("[.]").collect::<Vec<&str>>()[..3].join(" "))
                    .collect::<Vec<String>>())];

                helper::install_downgrade(package.split("[.]").collect::<Vec<&str>>()[3], args.file_path, args.keep);

            } else {
                if !args.search {
                    let (package_name, package_version, git_url) = helper::get_package(&args.package);

                    success(&format!("Package found:\x1b[1;37m {package_name} [{package_version}]"));

                    let exists = get_current_version(&package_name);

                    if exists != "" {
                        warning(&format!("Package already installed:\x1b[1;37m{exists}"));

                    }

                    helper::install_package(package_name, git_url, args.file_path, args.keep);
                }

                else {
                    let packages = helper::search_package(&args.package);

                    if packages.len() == 0 {
                        error("Invalid query.");

                    }

                    success(&format!("\x1b[1;37m{} \x1b[1;34mpackages found with query \x1b[1;37m{}", packages.len(), args.package));

                    let exists = get_current_version(&args.package);

                    if exists != "" {
                        warning(&format!("Package with same name already installed:\x1b[1;37m{exists}"));

                    }

                    let package = packages[helper::select_package(&packages)].clone();

                    prompt("Install package? [y/N]");

                    let package_name = package.split(" ").collect::<Vec<&str>>()[0];

                    helper::install_package(package_name.to_string(), format!("https://aur.archlinux.org/{}.git", package_name), args.file_path, args.keep);
                }
            }
        }

    } else {
        error("Package name and at least a mode is required.");

    }
}
