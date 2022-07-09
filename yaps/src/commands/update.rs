use crate::constants::{APPDB_PATH, MIRROR_PATH, PKG_DB_URL_PREFIX};

use std::fs::{create_dir_all, File};
use std::io::{prelude::*, BufReader};
use std::process::{exit, Command};

pub fn update() {
    let file = match File::open(MIRROR_PATH) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(_why) => {
            eprintln!("couldn't open");
            exit(1);
        }
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let _line: String = line.unwrap();
        let _split: Vec<&str> = _line.split("=").collect();
        let _mid: String = _split[0].to_string();
        let _murl: String = _split[1].to_string();
        println!("Updating {:#?}", _mid);
        yupdate_mirror(&_mid, &_murl);
    }
}

fn yupdate_mirror(mid: &str, murl: &str) {
    let _out = create_dir_all(APPDB_PATH);

    let output = Command::new("wget")
        .args([
            "-O",
            &format!("{APPDB_PATH}/{mid}"),
            &format!("{murl}/{PKG_DB_URL_PREFIX}/{mid}"),
        ])
        .output()
        .expect("failed to execute process");

    println!(
        "{}",
        if output.status.success() {
            "Updated successfully"
        } else {
            "Some error occured while updating..."
        }
    );
}
