use crate::functions::{list_depends::*, read_input::*, y_compiler::*};
use std::process::exit;

pub fn compile(
    app_id: String,
    no_depends: &bool,
    no_install: &bool,
    compiler_specs: &Option<String>,
) {
    if !no_depends {
        println!("Resolving dependencies...");
        let depends_list = list_depends(&app_id);
        let depcount = depends_list.len();

        if depcount > 1 {
            println!(
                "{:?}\nDo you want to install these dependencies [y/n]: ",
                depends_list,
            );
            if read_input() != "y" {
                eprintln!("Cancelled");
                exit(1);
            }
        }

        for i in depends_list {
            if !y_compiler(&i, *no_install, compiler_specs) {
                eprintln!("Failed to install {i}");
                exit(1);
            }
        }
    } else {
        if !y_compiler(&app_id, *no_install, compiler_specs) {
            eprintln!("Failed to install {app_id}");
            exit(1);
        }
    }
}
