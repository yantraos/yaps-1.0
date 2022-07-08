use crate::constants::*;
use crate::functions::{get_arch::*, run_tar::*, y_builder::*, y_installer::*, yy_build_path::*};
use std::process;

pub fn y_compiler(app_id: &String, no_install: bool, compiler_specs: &Option<String>) -> bool {
    let y_path = yy_build_path(app_id);
    if y_path.is_empty() {
        println!("No ybuild file found for {app_id}");
        process::exit(2);
    }

    println!("=> Found ybuild file $ypath");
    let pkg_tar = format!("$PKGDIR/$id-$version-$release-{}.{}", get_arch(), EXTENSION);

    if !y_builder(&format!("{y_path}/ybuild"), &pkg_tar, compiler_specs) {
        println!("Error Failed to compile {app_id}");
        process::exit(1);
    }

    if !no_install {
        if y_installer(&pkg_tar) != 0 {
            println!("Error Failed to install {}", app_id);
            process::exit(1);
        }

        run_tar(&pkg_tar);
    } else {
        println!("** Skipping installation **");
    }
    true
}
