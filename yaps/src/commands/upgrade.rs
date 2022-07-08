use crate::constants::{YDATADIR, ROOTDIR};
use std::fs::read_dir;
use crate::functions::{y_info::*, read_input::*};

pub fn upgrade() {
    let mut update_list : Vec<String> = vec![];
    // Iterate through files
    let paths = read_dir(format!("{ROOTDIR}/usr/share/{YDATADIR}/")).unwrap();
    for path in paths {

        let _p = path.unwrap().path().display().to_string();
        let app_id = _p;
        let _info = y_info(&app_id);
        let app_ver = _info.version;
        let app_rel = _info.release;

        // if ( "app_ver" =~ "->" ) || ( "app_rel" =~ "->" ) {
             update_list.push(app_id);
        // }
    }
    let update_count= update_list.len();
    if update_count > 0 {
        println!("Found ({update_count}) update");
        println!("{:?}\nDo you want to update [y|n]: ", update_list);

        if read_input() != "y" {
            println!("Cancelled");
        }

        for i in update_list {
            // export UPDATE=1
            //if ($COMPILE) {
            //    if y_installer(i) != 0 {
            //        panic!("Failed to update {i} ({ret})");
            //    }
            //    continue;
            //}
            // if y_compiler(i) != 0 {
            //     panic!("Failed to update {i} ({ret})");
            // }
        }
    }
    else {
        println!("System is upto date");
    }
}
