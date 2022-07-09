use std::fs::{remove_dir, remove_dir_all, remove_file, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

pub fn remove(app_id: String) {
    // Declare all constants
    const ROOTDIR: &str = "/";
    const YDATADIR: &str = "yaps";

    // Declare all variables
    let app_datadir = format!("{ROOTDIR}/usr/share/yaps/{app_id}");
    let info_file = format!("{app_datadir}/info");
    let files_list = format!("{app_datadir}/files");
    let install_script = format!("{app_datadir}/install");

    // Check if app data exist
    if !Path::new(&app_datadir).is_dir() {
        eprintln!("Error: Yaps data missing for {app_id}");
        exit(1);
    }

    // Check if info file exist
    if !Path::new(&info_file).is_file() {
        eprintln!("Error: Information file missing for {app_id}");
        exit(2);
    }

    // Check if files list exist
    if !Path::new(&files_list).is_file() {
        eprintln!("Error: Files list missing for {app_id}");
        exit(3);
    }

    // Check and execute preremove() function
    println!("=> Checking and executing pre-remove script");
    Command::new("bash")
        .args(["-c", &format!("\"source {install_script} && preremove\"")])
        .output()
        .unwrap()
        .stdout;

    // Remove installed files
    let mut file = BufReader::new(File::open(&files_list).expect("Unable to open file"));
    let count = file.lines().count();
    file = BufReader::new(File::open(&files_list).expect("Unable to open file"));

    println!("Cleaning {} files", &count);

    // Iterates through the files list
    for line in file.lines() {
        let _line = line.unwrap();

        // Checks if the line is a directory
        if Path::new(&_line).is_dir() {
            // Checks if the directory is empty
            if PathBuf::from(&_line).read_dir().unwrap().next().is_none() {
                remove_dir(&_line).expect("Directory cannot be deleted!");
            }
        // Else line is a file
        } else {
            remove_file(Path::new(&_line)).expect("File cannot be deleted!");
        }
    }

    // Check and execute preremove() function
    println!("=> Checking and executing post-remove script");
    Command::new("bash")
        .args(["-c", &format!("\"source {install_script} && postremove\"")])
        .output()
        .unwrap()
        .stdout;

    // Removes files list file
    remove_file(Path::new(&files_list)).expect("File cannot be deleted!");

    // Removes all other files related to app id
    remove_dir_all(Path::new(&format!(
        "{ROOTDIR}/usr/share/{YDATADIR}/{app_id}",
    )))
    .expect("Directory cannot be deleted!");

    // At last, return a success message
    println!("{app_id} removed successfully");
}
