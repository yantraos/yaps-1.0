use std::process::exit;
use crate::functions::{list_depends::*, y_installer::*, read_input::*};

pub fn install(app_id: String, no_depends: &bool){
    if !no_depends {
        println!("Resolving dependencies...");
        let depends_list = list_depends(&app_id);
        // Count number of dependencies
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
            y_installer(&i);
        }
    }

    y_installer(&app_id);
}
