use crate::constants::*;
use crate::functions::get_arch::*;
use std::fs;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::process::{exit, Command};

pub fn y_downloader(path: &String) -> u8 {
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
        println!("Downloading {_murl}/{path}");

        let link = Command::new("wget")
            .args([
                // WGETFLAGS,
                "-q",
                "--show-progress",
                &format!("{_murl}/{PKG_URL_PREFIX}/{}/{path}", get_arch()),
                "-O",
                &format!("{PKGDIR}/{path}.part"),
            ])
            .output()
            .expect("Failed to Execute Wget Command!");

        if !link.status.success() {
            eprintln!("Failed to {path} from {_mid} ({_murl})");
            exit(1);
        }

        fs::rename(format!("{PKGDIR}/{path}.part"), format!("{PKGDIR}/{path}"))
            .expect("Renaming failed!");

        return 0;
    }

    1
}
