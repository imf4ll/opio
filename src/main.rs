mod utils;
mod archive;
mod aur;
mod upgrade;

use crate::utils::{packages::*, log::*, banner::*, status::*};

use clap::Parser;
use nix::unistd::geteuid;

#[derive(Parser)]
#[command(author, version, about = about(), long_about = None)]
struct Args {
    /// Install first valid package from AUR
    #[arg(short, long, value_name = "PACKAGE", default_value = "")]
    install: String,

    /// Final package download path
    #[arg(short, long, value_name = "PATH", default_value = "")]
    file_path: String,

    /// Ignores packages from cache while downgrading 'pacman' packages
    #[arg(long, default_value_t = false)]
    ignore_cache: bool,

    /// Turns on downgrade mode
    #[arg(short, long, value_name = "PACKAGE", default_value = "")]
    downgrade: String,

    /// Turns on AUR helper mode
    #[arg(short, long, default_value_t = false)]
    aur: bool,

    /// Search for a package in AUR
    #[arg(short, long, value_name = "PACKAGE", default_value = "")]
    search: String,

    /// Check Archive and AUR status
    #[arg(long, default_value_t = false)]
    status: bool,

    /// Keep AUR package after installing
    #[arg(short, long, default_value_t = false)]
    keep: bool,

    /// Update a package from AUR to latest version
    #[arg(short, long, value_name = "PACKAGE", default_value = "")]
    update: String,

    /// Shows PKGBUILD before install
    #[arg(long, default_value_t = false)]
    pkgbuild: bool,

    /// Self update 'opio' to latest version
    #[arg(long, default_value_t = false)]
    upgrade: bool,

    /// Install package without prompt confirmation
    #[arg(long, default_value_t = false)]
    noconfirm: bool,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut args = Args::parse();

    banner();

    if args.file_path == "" {
        if args.downgrade != "" && !args.aur {
            args.file_path = "/var/cache/pacman/pkg/".to_string();

        } else {
            args.file_path = "/tmp/".to_string();

        }
    }

    if args.upgrade {
        upgrade::self_upgrade().await?;
        
        std::process::exit(2);

    } else if args.status {
        get_status().await?;
    
        std::process::exit(2);
    }
    
    if args.downgrade == "" && args.update == "" && args.install == "" && args.search == "" {
        error("A mode is required: downgrade, update, search or install (Check opio -h to more options)");

    } else if args.install != "" {
        wait("Searching package on \x1b[1;37maur.archlinux.org...");
        
        let (package_name, package_version, git_url) = aur::get_package(&args.install).await?;

        success(&format!("Package found:\x1b[1;37m {package_name} [{package_version}]"));

        let exists = get_current_version(&package_name);

        if exists != "" {
            warning(&format!("Package already installed:\x1b[1;37m{exists}"));

        }

        aur::install_package(package_name, git_url, args.file_path, args.keep, args.pkgbuild, args.noconfirm);
     
    } else if args.search != "" {
        wait("Searching package on \x1b[1;37maur.archlinux.org...");
        
        let packages = aur::search_package(&args.search).await?;

        if packages.len() == 0 {
            error("Invalid query.");

        }

        success(&format!("\x1b[1;37m{} \x1b[1;34mpackages found with query \x1b[1;37m{}", packages.len(), args.search));

        let exists = get_current_version(&args.search);

        if exists != "" {
            warning(&format!("Package with same name already installed:\x1b[1;37m{exists}"));

        }

        let package = packages[aur::select_package(&packages)].clone();
        
        let package_name = package.split(" ").collect::<Vec<&str>>()[0];
        
        aur::install_package(package_name.to_string(), format!("https://aur.archlinux.org/{}.git", package_name), args.file_path, args.keep, args.pkgbuild, args.noconfirm);
    
    } else if args.update != "" {
        wait("Searching for latest version on \x1b[1;37maur.archlinux.org...");

        let (package_name, package_version, git_url) = aur::get_package(&args.update).await?;

        let current_version = get_current_version(&args.update);

        if current_version.contains(&package_version) {
            error(&format!("Package already up to date.{current_version}"));

        }
        
        success(&format!(
            "Updating \x1b[1;37m{package_name} \x1b[1;34mfrom \x1b[1;37m{} \x1b[1;34mto \x1b[1;37m{package_version}",
            current_version
                .replace("]", "")
                .split("current: ")
                .collect::<Vec<&str>>()[1]
                .trim()
        ));

        aur::install_package(package_name, git_url, args.file_path, args.keep, args.pkgbuild, args.noconfirm);
    
    } else if args.downgrade != "" {
        if args.aur {
            wait("Searching package on \x1b[1;37maur.archlinux.org...");
            
            let packages = aur::get_downgrade(&args.downgrade).await?;

            success(&format!("Listing last \x1b[1;37m{} \x1b[1;34mversions.", packages.len()));

            if get_current_version(&args.downgrade) == "" {
                warning("Package not installed!");

            }

            let package = &packages[aur::select_package(&packages
                .clone()
                .into_iter()
                .map(| i | i.split("[.]").collect::<Vec<&str>>()[..3].join(" "))
                .collect::<Vec<String>>())];

            aur::install_downgrade(package.split("[.]").collect::<Vec<&str>>()[3], args.file_path, args.keep, args.pkgbuild, args.noconfirm)
                .await
                .expect(&format!("{ERROR} Failed to install downgrade."));
        
        } else {
            if !geteuid().is_root() {
                error("Superuser privileges needed.");
         
            }

            let package_subject = args.downgrade.chars().collect::<Vec<char>>()[0];
            let package_url = format!("https://archive.archlinux.org/packages/{}/{}", package_subject, args.downgrade);

            wait("Searching package on \x1b[1;37marchive.archlinux.org...");

            let packages = archive::get_package(&package_url, &args.downgrade, args.ignore_cache).await?;

            if get_current_version(&args.downgrade) == "" {
                warning("Package not installed!");

            }

            success(&format!("\x1b[1;37m{} \x1b[1;34mversions found.", packages.len()));
            
            let package = packages[archive::select_package(args.downgrade, &packages)].split(" ").collect::<Vec<&str>>();

            if package[1].contains("Cache") {
                archive::install_package(format!("/var/cache/pacman/pkg/{}", package[0]), args.noconfirm);

            } else {
                archive::download_package(args.file_path, &package[0], format!("{package_url}/{}", package[0]), args.noconfirm).await?;

            }
        }
    }

    Ok(())
}
