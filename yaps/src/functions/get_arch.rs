use std::process::Command;

pub fn get_arch() -> String {
    let output = String::from_utf8(
            Command::new("uname")
            .arg("-m")
            .output()
            .expect("Failed to execute 'uname -m' Command")
            .stdout
        ).unwrap();

    output
}
