use std::path::Path;
use std::process::{ Command, exit };
use crate::constants::*;
use crate::functions::run_tar::*;
use crate::functions::{get_arch::*, y_downloader::*};


pub fn y_installer (app_id: &String) -> u8 {
    let _aver="$(WITHOUTSYS=1 yinfo $appid 'version')";
    let _arel="$(WITHOUTSYS=1 yinfo $appid 'release')";

    let ypkg= format!("{app_id}-{_aver}-{_arel}-{}.{EXTENSION}", get_arch());
    if ! Path::new(format!("{PKGDIR}/{ypkg}").as_str()).exists() {
        println!("=> Downloading binary package");

        if y_downloader(&ypkg) != 0 {
            eprintln!("** Failed to download {app_id}");
            exit(1);
        }
    } else {
        println!("** found in cache");
    }


    let installer = Command::new(YINSTALLER).arg(
        format!("{PKGDIR}/{ypkg}"),
    )
    .output()
    .expect("Failed to Execute Command!");

    if !installer.status.success() {
        eprintln!("Error Failed to install {app_id}");
        exit(1);
    }

    run_tar(&format!("{PKGDIR}/{ypkg}"));

    0
}
