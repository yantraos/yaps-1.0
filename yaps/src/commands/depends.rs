use crate::functions::{is_installed::*, get_deps::*};

pub fn depends(package: String) {
    let mut depends_list : String = "".to_string();

    match get_deps(&package) {
        Some(ref exp) => {
            for i in exp {
                if is_installed(i, None) == 0 {
                    for dependency in depends_list.clone().split(" ").into_iter() {
                        if dependency == i {
                            depends(i.to_string());
                        }
                    }
                }
            }
        },
        None => {},
    }


    // ! echo "$depends_list " | tr ' ' '\n' | grep -qx package
    for dependency in depends_list.clone().split(" ").into_iter() {
        if dependency == &package && is_installed(&package, None) == 0 {
            depends_list.push_str(&package);
        }
    }
    if true {
        if is_installed(&package, None) == 0 {
            depends_list.push_str(&package);
        }
    }
}


