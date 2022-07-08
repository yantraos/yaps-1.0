use std::fs;
use std::path::Path;
use std::process::exit;
use crate::structs::YBuild;
use crate::functions::yy_build_path::*;

pub fn get_deps(package: &String) -> Option<Vec<String>> {
    let ypath : &str = &yy_build_path(package);

    if !Path::new(ypath).exists() {
        eprintln!("No ybuild file");
        exit(1);
    }

    let contents = fs::read_to_string(format!("{ypath}/ybuild.yaml"))
        .expect("Something went wrong reading the file");
    let yaml: YBuild = serde_yaml::from_str(contents.as_str()).unwrap();

    yaml.runtime
}
