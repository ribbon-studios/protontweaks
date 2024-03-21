use std::{
    env,
    fs::{self, OpenOptions},
    path::Path,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub gamemode: bool,
    #[serde(default)]
    pub mangohud: bool,
}

impl Config {
    pub fn off() -> Self {
        Self {
            gamemode: false,
            mangohud: false,
        }
    }
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

pub fn try_delete(path: Option<String>) -> Result<String, &'static str> {
    let path = path.ok_or("Path not provided")?;

    if fs::metadata(&path).is_ok() && fs::remove_file(&path).is_ok() {
        Ok(path)
    } else {
        Err("File not found!")
    }
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

pub fn detect_valid_home() -> Result<String, String> {
    let paths = vec![
        get_xdg_path("protontweaks.json"),
        get_home_path("protontweaks.json"),
    ];

    for path in paths {
        if let Some(path) = path {
            return Ok(path);
        }
    }

    Err("Failed to detect a valid home directory, are you sure either $XDG_CONFIG_HOME or $HOME are set?".to_string())
}

pub fn load() -> Config {
    let paths = vec![
        get_xdg_path("protontweaks.json"),
        get_home_path("protontweaks.json"),
        get_etc_path("protontweaks.json"),
    ];

    for path in paths {
        if let Some((path, config)) = try_load(path) {
            info!("Loaded config from {path}...");
            return config;
        }
    }

    info!("No config found, loading default config!");
    Config::default()
}

pub fn try_save(path: &String, config: &Config) -> Result<(), String> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&path)
        .map_err(|e| e.to_string())?;

    serde_json::to_writer_pretty(&file, config).map_err(|e| e.to_string())?;

    Ok(())
}

pub fn wipe() {
    let paths = vec![
        get_xdg_path("protontweaks.json"),
        get_home_path("protontweaks.json"),
        get_etc_path("protontweaks.json"),
    ];

    for path in paths {
        if let Ok(path) = try_delete(path) {
            info!("Cleaned up config located at {path}.");
        }
    }
}
