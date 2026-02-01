use std::{fs::{self}, io::ErrorKind};

use crate::settings::Settings;


pub async fn get_list() -> Result<Vec<String>, ()> {
    let settings = Settings::new().expect("message");
    let list_path = settings.data.list_path;
    let filename = format!("{}list.txt", &list_path);

    match fs::read_to_string(filename) {
        Ok(contents) => {
            Ok(contents.lines().map(String::from).collect())
        }
        Err(e) if e.kind() == ErrorKind::NotFound => {
            Ok(Vec::new())
        }
        Err(e) => {
            eprintln!("Error when reading file: {}", e);
            Err(())
        },
    }
}

pub async fn update_list(ip_list: &[String]) -> Result<(), ()> {
    let settings = Settings::new().expect("message");
    let list_path = settings.data.list_path;
    let filename = format!("{}list.txt", &list_path);
    let _ = fs::write(filename, ip_list.join("\n"));
    Ok(())
}
