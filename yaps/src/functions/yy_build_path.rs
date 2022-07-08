use std::fs;
use std::path::Path;
use crate::constants::*;

pub fn yy_build_path(_package: &String) -> String {
    let paths = fs::read_dir(REPOSITORY).unwrap();

    for path in paths {
        let _path = fs::read_dir(path.unwrap().path()).unwrap();
        for j in _path {
            let _p = j.unwrap().path().display().to_string();
            match Path::new(&_p).file_stem().unwrap() {
                _package => return _p,
            }
        }
    }

    "".to_string()
}
