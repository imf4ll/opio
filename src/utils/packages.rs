use std::process::Command;

use crate::utils::log::ERROR;

pub fn get_current_version(package: &String) -> String {
    let version = Command::new("sudo")
        .arg("pacman")
        .arg("-Q")
        .arg(package)
        .output()
        .expect(&format!("{ERROR} Failed to run pacman as sudo, are you rooted?"))
        .stdout;

    let stdout = String::from_utf8(version).unwrap();

    if stdout != "" {
        format!(" [ current: {} ]", stdout.trim().split(" ").collect::<Vec<&str>>()[1])
            
    } else {
        "".to_string()

    }
}

pub fn reversed(vector: &Vec<String>) -> Vec<String> {
    let mut reverse = vec![];

    vector
        .into_iter()
        .rev()
        .for_each(| i | reverse.push(i.to_string()));

    reverse
}

pub fn get_file_size(file: &std::fs::File) -> u64 {
    file
        .metadata()
        .expect(&format!("{ERROR} Failed to get file size, are you rooted?"))
        .len()
}
