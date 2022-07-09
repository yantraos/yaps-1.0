use crate::functions::y_info::*;
use crate::structs::YInfo;

pub fn info(package: String) {
    let _info: YInfo = y_info(&package);

    println!("Name         : {}", _info.name);
    println!("Version      : {}", _info.version);
    println!("Release      : {}", _info.release);

    match _info.description {
        Some(description) => println!("Description  : {}", description),
        None => {}
    }

    match _info.depends {
        Some(depends) => println!("Depends on   : {}", depends),
        None => {}
    }
}
