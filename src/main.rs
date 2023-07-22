mod utils;
mod downgrade;
mod helper;

use std::io::Write;

use crate::utils::{packages::*, log::*, banner::*};

use clap::Parser;
use nix::unistd::geteuid;

#[derive(Parser)]
#[command(author, version, about = about(), long_about = None)]
struct Args {
    /// Package name
    #[arg(short, long, value_name = "PACKAGE")]
    package: String,

    /// Final package download path
    #[arg(short, long, value_name = "PATH", default_value = "/var/cache/pacman/pkg")]
    file_path: String,

    /// Ignores packages from cache
    #[arg(short, long, default_value_t = false)]
    ignore_cache: bool,

    /// Runs in downgrade mode
    #[arg(short, long, default_value_t = false)]
    downgrade: bool,

    /// Runs in AUR helper mode
    #[arg(short, long, default_value_t = true)]
    aur: bool,

    /// Search for a package in AUR
    #[arg(short, long, default_value_t = false)]
    search: bool,
}

fn main() {
    let args = Args::parse();

    banner();

    if args.package != "" {
        if args.downgrade {
            if !geteuid().is_root() {
                error("Superuser privileges needed.");
         
            }

            let package_subject = args.package.chars().collect::<Vec<char>>()[0];
            let package_url = format!("https://archive.archlinux.org/packages/{}/{}", package_subject, args.package);
            
            let packages = downgrade::get_package(&package_url, &args.package, args.ignore_cache);

            if get_current_version(&args.package) == "" {
                println!("\x1b[1;33m⚠ Package not installed!\x1b[m\n");

            }
            
            println!("\x1b[1;34m✔ \x1b[1;37m{} \x1b[1;34mversions found.\x1b[m\n", packages.len());
            
            let package = packages[downgrade::select_package(args.package, &packages)].split(" ").collect::<Vec<&str>>();

            if package[1].contains("Cache") {
                downgrade::install_package(format!("{}/{}", args.file_path, &package[0]));

            } else {
                downgrade::download_package(args.file_path, &package[0], format!("{package_url}/{}", package[0]))

            }
        
        } else if args.aur {
            if !args.search {
                let package_url = format!("https://aur.archlinux.org/packages/{}", args.package);
            
                let (package_name, package_version, git_url) = helper::get_package(package_url);

                println!("\x1b[1;34m✔ Package found:\x1b[1;37m {package_name} [{package_version}]\x1b[m\n");

                let exists = get_current_version(&package_name);

                if exists != "" {
                    println!("\x1b[1;33m⚠ Package already installed:\x1b[1;37m{exists}\x1b[m");

                }

                let mut confirm = String::new();

                print!("\n\x1b[1;33m❓Install package? [y/N]\x1b[m ");

                std::io::stdout().flush().unwrap();

                std::io::stdin().read_line(&mut confirm).unwrap();

                if confirm.trim().to_lowercase() != "y" {
                    println!("\nAborted by user.");

                    std::process::exit(2);
                }

                println!("");

                helper::install_package(package_name, git_url);
            }

            else {
                let packages = helper::search_package(&args.package);

                if packages.len() == 0 {
                    error("Invalid query.");

                }

                println!("\x1b[1;34m✔ \x1b[1;37m{} \x1b[1;34mpackages found with query \x1b[1;37m{}\x1b[m\n", packages.len(), args.package);

                let exists = get_current_version(&args.package);

                if exists != "" {
                    println!("\x1b[1;33m⚠ Package with same name already installed:\x1b[1;37m{exists}\x1b[m\n");

                }

                let package = packages[helper::select_package(&packages)].clone();

                let mut confirm = String::new();

                print!("\n\x1b[1;33m❓Install package? [y/N]\x1b[m ");

                std::io::stdout().flush().unwrap();

                std::io::stdin().read_line(&mut confirm).unwrap();

                if confirm.trim().to_lowercase() != "y" {
                    println!("\nAborted by user.");

                    std::process::exit(2);
                }

                println!("");

                let package_name = package.split(" ").collect::<Vec<&str>>()[0];

                helper::install_package(package_name.to_string(), format!("https://aur.archlinux.org/{}.git", package_name));
            }
        }

    } else {
        error("Package name is required.");

    }
}
