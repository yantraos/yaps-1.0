use std::io::prelude::*;
use std::process::{Command, Stdio};
use crate::constants::*;

pub fn run_tar(path: &String) {
    let mut cmd_tar = Command::new("tar")
        .args(
            [
                "-tf",
                path,
            ],
        )
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let mut cmd_trigger = Command::new(YTRIGGERER)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();


    if let Some(ref mut stdout) = cmd_tar.stdout {
        if let Some(ref mut stdin) = cmd_trigger.stdin {
            let mut buf: Vec<u8> = Vec::new();
            stdout.read_to_end(&mut buf).unwrap();
            stdin.write_all(&buf).unwrap();
        }
    }

    let res = String::from_utf8(cmd_trigger.wait_with_output().unwrap().stdout).unwrap();
    println!("{res}");
}
