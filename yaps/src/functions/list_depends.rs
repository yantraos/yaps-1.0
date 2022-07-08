use crate::functions::{get_deps::*, is_installed::*};

pub fn list_depends (app_id: &String) -> Vec<String> {
    let mut depends_list : Vec<String> = vec![];

    match get_deps(&app_id) {
        Some(deps) => {
            for i in deps {
                if is_installed(&i, None) == 1 {
                    if ! depends_list.contains(&i) {
                        list_depends(&i);
                    }
                }
            }
        },
        None => {},
    }


    if ! depends_list.contains(app_id) {
        if is_installed(&app_id, None) == 1 {
            depends_list.push(app_id.to_string());
        }
    }

    depends_list
}
