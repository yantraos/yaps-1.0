use crate::functions::yy_build_path::*;
use std::path::Path;
use std::process::{exit, Command};

pub fn get_deps(package: &str) -> Vec<String> {
    let ypath: &str = &yy_build_path(&package.to_string());

    if !Path::new(ypath).exists() {
        eprintln!("No ybuild file");
        exit(1);
    }

    let runtime = String::from_utf8(
        Command::new("bash")
            .args(["-c", &format!("\"source {ypath}/ybuild && echo $runtime\"")])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap()
    .split_whitespace()
    .collect::<Vec<&str>>()
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    runtime
}
