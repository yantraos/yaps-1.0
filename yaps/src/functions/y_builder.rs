use std::env;
use std::process::Command;

pub fn y_builder(script_file: &String, pkg_tar: &String, compiler_specs: &Option<String>) -> bool {
    match compiler_specs {
        Some(val) => {
            env::set_var("COMPILER_SPECS", val);
        }
        None => {}
    }
    let cm = Command::new("yaps.build")
        .args([script_file, pkg_tar])
        .status();

    return cm.unwrap().success();
}
