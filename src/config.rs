use std::{env, fs, path::Path};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub gamemode: bool,
    #[serde(default)]
    pub mangohud: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            gamemode: true,
            mangohud: false,
        }
    }
}

pub fn get_path(root: Option<String>, paths: Vec<&str>) -> Option<String> {
    let root = root?;
    let mut buf = Path::new(root.as_str()).to_path_buf();

    for path in paths {
        buf = buf.join(path);
    }

    Some(buf.to_str()?.to_string())
}

pub fn get_xdg_path(file: &str) -> Option<String> {
    get_path(env::var("XDG_CONFIG_HOME").ok(), vec![file])
}

pub fn get_home_path(file: &str) -> Option<String> {
    get_path(env::var("HOME").ok(), vec![".config", file])
}

pub fn get_etc_path(file: &str) -> Option<String> {
    get_path(Some("/etc".to_string()), vec![file])
}

pub fn try_load(path: Option<String>) -> Option<(String, Config)> {
    let path = path?;

    info!("Checking {path} for config...");

    if fs::metadata(&path).is_ok() {
        let raw_config = fs::read_to_string(&path).ok()?;
        let config = serde_json::from_str::<Config>(&raw_config).ok()?;
        Some((path, config))
    } else {
        info!("Not found!");
        None
    }
}

pub fn load() -> Config {
    if let Some((path, config)) = try_load(get_xdg_path("protontweaks.json")) {
        info!("Loaded config from {path}...");
        config
    } else if let Some((path, config)) = try_load(get_home_path("protontweaks.json")) {
        info!("Loaded config from {path}...");
        config
    } else if let Some((path, config)) = try_load(get_etc_path("protontweaks.json")) {
        info!("Loaded config from {path}...");
        config
    } else {
        info!("No config found, loading default config!");
        Config::default()
    }
}
