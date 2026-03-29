// vim: fdm=indent fdn=1
//
// Contains all the logic necessary to handle the internal peer list
// of a node

use crate::settings::Settings;
use std::{
    collections::HashSet,
    fs::{self},
    io::{ErrorKind, Write},
};

const LIST_FILENAME: &str = "list.txt";

pub fn get_peer_list() -> Result<HashSet<String>, ()> {
    let settings = Settings::new().expect("message");
    let list_path = settings.data.list_path;
    let filename = format!("{}{}", &list_path, LIST_FILENAME);

    match fs::read_to_string(filename) {
        Ok(contents) => Ok(contents.lines().map(String::from).collect()),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(HashSet::new()),
        Err(e) => {
            eprintln!("Error when reading file: {}", e);
            Err(())
        }
    }
}

pub fn update_peer_list(ip_list: Vec<String>) -> Result<(), ()> {
    let settings = Settings::new().expect("message");
    let list_path = settings.data.list_path;
    let filename = format!("{}{}", &list_path, LIST_FILENAME);
    let _ = fs::write(filename, ip_list.join("\n"));
    Ok(())
}

pub fn append_peer_list(ip_list: Vec<String>) -> Result<(), ()> {
    let settings = Settings::new().expect("message");
    let list_path = settings.data.list_path;
    let filename = format!("{}{}", &list_path, LIST_FILENAME);

    if let Ok(mut fh) = fs::OpenOptions::new().append(true).open(filename) {
        let _ = writeln!(&mut fh, "\n{}", ip_list.join("\n"));
    }
    Ok(())
}
