use crate::constants::*;
use crate::structs::YInfo;
use std::fs;
use std::io::{prelude::*, BufReader};
use std::process::exit;

pub fn y_info(package: &String) -> YInfo {
    let paths = fs::read_dir(format!("{APPDB_PATH}")).unwrap();

    for path in paths {
        let file = match fs::File::open(format!("{}", path.unwrap().path().display())) {
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
            let _split: Vec<&str> = _line.split(" ").collect();
            let _name: String = _split[0].to_string();
            if _name == package.as_str() {
                let _version: String = _split[1].to_string();
                let _release: String = _split[2].to_string();
                let mut _description: Option<String> = None;
                let mut _depends: Option<String> = None;

                if _split.len() >= 4 {
                    _description = Some(_split[3].to_string());

                    if _split.len() >= 5 {
                        _depends = Some(_split[4].to_string());
                    }
                }

                let _info: YInfo = YInfo {
                    name: _name,
                    version: _version,
                    release: _release,
                    description: _description,
                    depends: _depends,
                };

                return _info;
            }
        }
    }

    eprintln!("Package not found!");
    exit(1);
}
