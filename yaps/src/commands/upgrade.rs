use crate::constants::{ROOTDIR, YDATADIR};
use crate::functions::{read_input::*, y_compiler::*, y_info::*, y_installer::*, yy_build_path::*};
use std::env::var;
use std::fs::read_dir;
use std::process::Command;

pub fn upgrade(compiler_specs: &Option<String>) {
    let mut update_list: Vec<String> = vec![];
    // Iterate through files
    let paths = read_dir(format!("{ROOTDIR}/usr/share/{YDATADIR}/")).unwrap();
    for path in paths {
        let _p = path.unwrap().path().display().to_string();
        let app_id = _p;
        let _info = y_info(&app_id);
        // let app_ver = _info.version;
        let app_rel = _info.release;
        let ypath: &str = &yy_build_path(&_info.name);
        let rel = String::from_utf8(
            Command::new("bash")
                .args(["-c", &format!("\"source {ypath}/ybuild && echo $release\"")])
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap();

        if app_rel.parse::<i16>().unwrap() < rel.parse::<i16>().unwrap() {
            update_list.push(app_id);
        }
    }
    let update_count = update_list.len();
    if update_count > 0 {
        println!("Found ({update_count}) update");
        println!("{:?}\nDo you want to update [y|n]: ", update_list);

        if read_input() != "y" {
            println!("Cancelled");
        }

        for i in update_list {
            match var("COMPILE") {
                Err(_err) => {
                    if y_installer(&i) != 0 {
                        panic!("Failed to update {i}");
                    }
                    continue;
                }
                Ok(_val) => {
                    if y_compiler(&i, false, compiler_specs) {
                        panic!("Failed to update {i}");
                    }
                }
            }
        }
    } else {
        println!("System is upto date");
    }
}
