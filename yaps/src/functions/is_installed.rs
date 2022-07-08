use crate::constants::*;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::process::exit;

// isInstalled(<name>, <?version>)
// ExitCode: -1  : <name> is not defined
//           -2  : <version> not defined in our info file
//           1   : not installed
//           0   : installed
//           2   : different version is installed
pub fn is_installed(name: &String, version: Option<String>) -> i32 {
    if !Path::new(&format!("/usr/share/{YDATADIR}/{name}/info")).exists() {
        return 1;
    }

    match version {
        Some(ref _x) => {}
        None => return 0,
    }

    let mut _installed_ver: String = "".to_string();

    // /usr/share/$YDATADIR/$_name/info | grep ^version | awk '{print $2}'

    let file = match File::open(format!("/usr/share/{YDATADIR}/{name}/info")) {
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
        let _temp: String = _split[0].to_string();

        if _temp == "version" {
            _installed_ver = _split[1].to_string();
        }
    }

    if _installed_ver != version.unwrap() {
        return 2;
    }

    0
}
