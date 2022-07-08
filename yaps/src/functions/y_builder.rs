use std::process::Command;

pub fn y_builder(script_file: &String, pkg_tar: &String) -> bool {
    let cm = Command::new("yaps.build")
        .args(
            [
                script_file,
                pkg_tar,
            ]
        ).status();
        
    return cm.unwrap().success();
}
