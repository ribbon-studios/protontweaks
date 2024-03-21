use std::{
    env,
    fs::{self, OpenOptions},
    path::Path,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(skip)]
    path: Option<String>,
    #[serde(default)]
    pub gamemode: bool,
    #[serde(default)]
    pub mangohud: bool,
}

fn get_path(root: &str, paths: Vec<&str>) -> String {
    let mut buf = Path::new(root).to_path_buf();

    for path in paths {
        buf = buf.join(path);
    }

    buf.to_str().expect("Failed to convert path.").to_string()
}

impl Config {
    /// The $HOME path for protontweaks.json
    pub fn home() -> Option<String> {
        let home = env::var("HOME").ok()?;

        Some(get_path(&home, vec![".config", "protontweaks.json"]))
    }

    /// The $XDG_CONFIG_HOME path for protontweaks.json
    pub fn xdg() -> Option<String> {
        let home = env::var("XDG_CONFIG_HOME").ok()?;

        Some(get_path(&home, vec!["protontweaks.json"]))
    }

    /// Returns a path to either the $XDG_CONFIG_HOME or $HOME protontweak paths if either variable is set.
    pub fn discover_valid_home() -> Result<String, String> {
        let paths = vec![Config::xdg(), Config::home()];

        for path in paths {
            if let Some(path) = path {
                return Ok(path);
            }
        }

        Err("Failed to detect a valid home directory, are you sure either $XDG_CONFIG_HOME or $HOME are set?".to_string())
    }

    /// The /etc path for protontweaks.json
    pub fn etc() -> String {
        get_path("/etc", vec!["protontweaks.json"])
    }

    /// Returns a list of all valid protontweak config paths
    pub fn all() -> Vec<String> {
        let mut paths: Vec<String> = vec![Config::xdg(), Config::home()]
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.clone().unwrap())
            .collect();

        paths.push(Config::etc());

        paths
    }

    /// Returns true if this config was loaded from the file system or persisted to the file system.
    pub fn persisted(&self) -> bool {
        self.path.is_some()
    }

    /// Searchs $XDG_CONFIG_HOME, $HOME/.config, and /etc for a protontweaks.json file.
    /// If one isn't discovered it returns the default config
    pub fn discover() -> Config {
        for path in Config::all() {
            if let Ok(config) = Config::load(&path) {
                return config;
            }
        }

        info!("No config found, loading default config!");
        Config::default()
    }

    /// Deletes all protontweak configs
    pub fn wipe() -> Result<(), String> {
        for path in Config::all() {
            if let Ok(mut config) = Config::load(&path) {
                config.delete()?;
            }
        }

        Ok(())
    }

    /// Loads a config at the given path
    pub fn load(path: &str) -> Result<Self, String> {
        info!("Checking {path} for config...");

        if fs::metadata(&path).is_ok() {
            let raw_config = fs::read_to_string(&path).map_err(|e| e.to_string())?;
            let mut config =
                serde_json::from_str::<Config>(&raw_config).map_err(|e| e.to_string())?;
            config.path = Some(path.to_string());
            Ok(config)
        } else {
            Err("File does not exist!".to_string())
        }
    }

    /// Deletes the current config if it is persisted
    pub fn delete(&mut self) -> Result<(), &'static str> {
        let path = self.path.as_ref().ok_or("Path not provided")?;

        if fs::metadata(&path).is_ok() && fs::remove_file(&path).is_ok() {
            info!("Deleted config located at '{path}'.");
            self.path = None;
            Ok(())
        } else {
            Err("File does not exist!")
        }
    }

    /// Saves the config if it was previously loaded or saved
    pub fn save(&mut self) -> Result<(), String> {
        let path = self
            .path
            .as_ref()
            .ok_or("Please run 'save_at' for the initial save.".to_string())?;

        self.save_at(&path.clone())
    }

    /// Saves the config in the home directory
    pub fn save_at_home(&mut self) -> Result<(), String> {
        let home = Config::home()
            .ok_or("Failed to save to the home directory as $HOME is not set!".to_string())?;

        self.save_at(&home)
    }

    /// Saves the config in the XDG_CONFIG_HOME directory
    pub fn save_at_xdg(&mut self) -> Result<(), String> {
        let home = Config::xdg().ok_or(
            "Failed to save to the xdg config home directory as $XDG_CONFIG_HOME is not set!"
                .to_string(),
        )?;

        self.save_at(&home)
    }

    /// Saves the config in the /etc directory
    pub fn save_at_etc(&mut self) -> Result<(), String> {
        self.save_at(&Config::etc())
    }

    /// Saves the config at the specified directory
    pub fn save_at(&mut self, path: &str) -> Result<(), String> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&path)
            .map_err(|e| e.to_string())?;

        serde_json::to_writer_pretty(&file, &self).map_err(|e| e.to_string())?;

        self.path = Some(path.to_string());

        Ok(())
    }

    /// Returns a config with all the options off
    pub fn off() -> Self {
        Self {
            path: None,
            gamemode: false,
            mangohud: false,
        }
    }
}

impl PartialEq for Config {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
            && self.gamemode == other.gamemode
            && self.mangohud == other.mangohud
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            path: None,
            gamemode: true,
            mangohud: false,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn get_test_file(name: &str) -> String {
        fs::create_dir_all("tests/.configs").unwrap();

        format!("tests/.configs/{}.json", name)
    }

    #[test]
    pub fn save_at() -> Result<(), String> {
        let file_name = get_test_file("save-at");
        let mut expected_config = Config::default();

        expected_config.save_at(&file_name)?;

        let actual_config = Config::load(&file_name)?;

        assert_eq!(expected_config, actual_config);

        Ok(())
    }

    #[test]
    pub fn save() -> Result<(), String> {
        let file_name = get_test_file("save");
        let mut expected_config = Config::default();

        expected_config.save_at(&file_name)?;

        expected_config.gamemode = false;

        expected_config.save()?;

        let actual_config = Config::load(&file_name)?;

        assert_eq!(expected_config, actual_config);

        Ok(())
    }

    #[test]
    pub fn delete() -> Result<(), String> {
        let file_name = get_test_file("delete");
        let mut expected_config = Config::default();

        expected_config.save_at(&file_name)?;

        Config::load(&file_name).expect("Config should exist");

        expected_config.delete()?;

        Config::load(&file_name).expect_err("Config should not exist");

        Ok(())
    }

    #[test]
    pub fn discover_default() -> Result<(), String> {
        // Not a fan of this, but not sure of a better way of testing this
        let mut previous_config = Config::discover();
        Config::wipe()?;

        let config = Config::discover();

        if previous_config.persisted() {
            previous_config.save()?;
        }

        assert_eq!(Config::default(), config);

        Ok(())
    }

    #[test]
    pub fn default() {
        assert_eq!(
            Config::default(),
            Config {
                path: None,
                gamemode: true,
                mangohud: false,
            }
        );
    }
}
