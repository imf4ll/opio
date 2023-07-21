mod utils;
mod packages;

use crate::{packages::*, utils::{packages::*, log::*, banner::*}};

use clap::Parser;
use nix::unistd::geteuid;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
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
}

fn main() {
    let args = Args::parse();
    
    if !geteuid().is_root() {
        error("Superuser privileges needed.");
 
    }

    banner();

    if args.package != "" {
        let package_subject = args.package.chars().collect::<Vec<char>>()[0];
        let package_url = format!("https://archive.archlinux.org/packages/{}/{}", package_subject, args.package);
        
        let packages = get_package(&package_url, &args.package, args.ignore_cache);
        let package = packages[select_package(args.package, &packages)].split(" ").collect::<Vec<&str>>();

        if package[1].contains("Cache") {
            install_package(format!("{}/{}", args.file_path, &package[0]));

        } else {
            download_package(args.file_path, &package[0], format!("{package_url}/{}", package[0]))

        }

    } else {
        error("Package name is required.");

    }
}
