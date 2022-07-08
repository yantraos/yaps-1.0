use crate::functions::{get_deps::*, is_installed::*};

pub fn depends(package: String) {
    let mut depends_list: String = "".to_string();

    for i in get_deps(&package) {
        if is_installed(&i, None) == 0 {
            for dependency in depends_list.split(" ").into_iter() {
                if dependency == i {
                    depends(i.to_string());
                }
            }
        }
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
