use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Ports {
    pub local: String,
    pub remote: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub(crate) struct Settings {
    pub ports: Ports,
}

impl Settings {
    pub(crate) fn new() -> Result<Self, ConfigError> {
        let home = std::env::var("HOME").expect("home does not exist");
        let default_path = format!("{}/.config/overheard/config.toml", &home);
        let config_path = env::var("OVERHEARD_CONFIG_PATH").unwrap_or(default_path);
        let s = Config::builder()
            .add_source(File::with_name(&config_path))
            .build()?;
        s.try_deserialize()
    }
}
