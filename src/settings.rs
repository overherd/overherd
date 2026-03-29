use config::{Config, ConfigError, File};
use std::sync::OnceLock;
use serde::Deserialize;
use std::env;

static SETTINGS: OnceLock<Settings> = OnceLock::new();

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Ports {
    pub local: String,
    pub remote: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Data {
    pub list_path: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Gossip {
    pub peer_refresh_rate: u64,
    pub peer_refresh_cancel_timeout: u64,
    pub max_peers: usize,
    pub peer_sample: usize,
    pub learn_requesting_peer_ratio: f64
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub ports: Ports,
    pub data: Data,
    pub gossip: Gossip,
}

impl Settings {
    pub(crate) fn get() -> &'static Settings {
        SETTINGS.get_or_init(|| {
            Settings::load().expect("Failed to load settings")
        })
    }

    fn load() -> Result<Self, ConfigError> {
        let home = std::env::var("HOME").expect("home does not exist");
        let default_path = format!("{}/.config/overheard/config.toml", &home);
        let config_path = env::var("OVERHEARD_CONFIG_PATH").unwrap_or(default_path);
        let s = Config::builder()
            .set_default("ports.local", "9999")?
            .set_default("ports.remote", "8080")?
            .set_default("data.list_path", format!("{}/.config/overheard/", &home))?
            .set_default("gossip.peer_refresh_rate", 8)?
            .set_default("gossip.peer_refresh_cancel_timeout", 2)?
            .set_default("gossip.max_peers", 4)?
            .set_default("gossip.peer_sample", 2)?
            .set_default("gossiplearn_requesting_peer_ratiolocal", 0.2)?
            .add_source(File::with_name(&config_path).required(false))
            .build()?;
        s.try_deserialize()


    }
}
