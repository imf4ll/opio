use std::process::Command;

pub fn read_pkgbuild(path: &str) {
    Command::new("vim")
        .arg(path)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
