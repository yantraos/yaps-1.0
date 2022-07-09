use crate::constants::APPDB_PATH;
use std::fs;
use std::io::{prelude::*, BufReader};
use std::process::exit;

pub fn list() {
    let paths = fs::read_dir(format!("{APPDB_PATH}")).unwrap();

    for path in paths {
        let file = match fs::File::open(format!("{}", path.unwrap().path().display())) {
            // The `description` method of `io::Error` returns a string that describes the error
            Err(_why) => {
                eprintln!("Couldn't open");
                exit(1);
            }
            Ok(file) => file,
        };

        let reader = BufReader::new(file);

        for line in reader.lines() {
            let _line: String = line.unwrap();
            let _split: Vec<&str> = _line.split(" ").collect();
            let name: String = _split[0].to_string();
            if _split.len() >= 4 {
                let description: String = _split[3].to_string();
                println!("{}: {}", name, description);
            } else {
                println!("{}", name);
            }
        }
    }
}
