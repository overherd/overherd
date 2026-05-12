// vim: fdm=indent fdn=1
//
// Contains all the logic necessary to handle the internal peer list
// of a node

use crate::settings::Settings;
use std::{
    collections::HashSet,
    fs::{self},
    io::{ErrorKind, Write},
    path::{Path, PathBuf},
};

fn get_list_path() -> PathBuf {
    let settings = Settings::get();
    Path::new(&settings.data.list_path).join(&settings.data.list_name)
}

/// Return current list of peers
pub fn get_peer_list() -> Result<HashSet<String>, ()> {
    match fs::read_to_string(get_list_path()) {
        Ok(contents) => Ok(contents.lines().map(String::from).collect()),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(HashSet::new()),
        Err(e) => {
            eprintln!("Error when reading file: {}", e);
            Err(())
        }
    }
}

/// Replace current peer list with the given one
pub fn update_peer_list(ip_list: Vec<String>) -> Result<(), ()> {
    if ip_list.len() == 0 {
        return Ok(());
    }
    let _ = fs::write(get_list_path(), ip_list.join("\n"));
    Ok(())
}

/// Append peer list to the existing one
pub fn append_peer_list(ip_list: Vec<String>) -> Result<(), ()> {
    if let Ok(mut fh) = fs::OpenOptions::new().append(true).open(get_list_path()) {
        let _ = writeln!(&mut fh, "\n{}", ip_list.join("\n"));
    }
    Ok(())
}
