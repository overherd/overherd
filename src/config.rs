use std::env;


pub fn local_port() -> String {
    env::var("LOCAL_PORT").unwrap_or("9999".to_string())
}

pub fn remote_port() -> String {
    env::var("REMOTE_PORT").unwrap_or("8080".to_string())
}

