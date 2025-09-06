use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};
use std::collections::BTreeMap;

/// Main configuration structure for LUBIG.
/// Stores directory paths and registry maps for repositories.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub directories: Directories,              // Paths for sources, profiles, and programs
    #[serde(default)]
    pub added: BTreeMap<String, String>,       // Registered repositories
    #[serde(default)]
    pub unlocked: BTreeMap<String, String>,    // Unlocked repos for updates (with branch)
    #[serde(default)]
    pub build: BTreeMap<String, String>,       // Built repos and their output paths
}

/// Directory paths used by LUBIG.
/// Defaults are relative to the executable location.
#[derive(Debug, Serialize, Deserialize)]
pub struct Directories {
    pub sources: Option<String>,
    pub profiles: Option<String>,
    pub programs: Option<String>,
}

impl Default for Directories {
    fn default() -> Self {
        Self {
            sources: Some(exe_path("sources").into()),
            profiles: Some(exe_path("profiles").into()),
            programs: Some(exe_path("programs").into()),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            directories: Directories::default(),
            added: BTreeMap::new(),
            unlocked: BTreeMap::new(),
            build: BTreeMap::new(),
        }
    }
}

impl Config {
    /// Loads configuration from `config.toml` if it exists, otherwise returns defaults.
    pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
        let path: PathBuf = conf_path();

        if path.exists() {
            let data = fs::read_to_string(&path)?;
            let cfg: Config = toml::from_str(&data)?;
            Ok(cfg)
        } else {
            Ok(Config::default())
        }
    }

    /// Modifies a section/key in the config and saves it back to disk.
    pub fn modify_and_save(&mut self, section: &str, key: &str, val: &str) -> std::io::Result<()> {
        match section {
            "Directories" => match key {
                "sources" => self.directories.sources = Some(val.to_string()),
                "profiles" => self.directories.profiles = Some(val.to_string()),
                "programs" => self.directories.programs = Some(val.to_string()),
                _ => eprintln!("Unknown key"),
            },
            "Added" => { self.added.insert(key.to_string(), val.to_string()); }
            "Unlocked" => { self.unlocked.insert(key.to_string(), val.to_string()); }
            "Build" => { self.build.insert(key.to_string(), val.to_string()); }
            _ => eprintln!("Unknown section"),
        }
        save(self)
    }

    /// Removes a key from a section and saves the config.
    pub fn remove_and_save(&mut self, section: &str, key: &str) -> std::io::Result<()> {
        match section {
            "Added" => { self.added.remove(key); }
            "Unlocked" => { self.unlocked.remove(key); }
            "Build" => { self.build.remove(key); }
            _ => eprintln!("Unknown section"),
        }
        save(self)
    }
    
    /// Retrieves a value from a given section/key.
    pub fn get_value(&self, section: &str, key: &str) -> Option<String> {
        match section {
            "Directories" => match key {
                "sources" => self.directories.sources.clone(),
                "profiles" => self.directories.profiles.clone(),
                "programs" => self.directories.programs.clone(),
                _ => None,
            },
            "Added" => self.added.get(key).cloned(),
            "Unlocked" => self.unlocked.get(key).cloned(),
            "Build" => self.build.get(key).cloned(),
            _ => None,
        }
    }

    /// Checks if a key exists in a given section.
    pub fn key_exists(&self, section: &str, key: &str) -> bool {
        match section {
            "Directories" => match key {
                "sources" => self.directories.sources.is_some(),
                "profiles" => self.directories.profiles.is_some(),
                "programs" => self.directories.programs.is_some(),
                _ => false,
            },
            "Added" => self.added.contains_key(key),
            "Unlocked" => self.unlocked.contains_key(key),
            "Build" => self.build.contains_key(key),
            _ => false,
        }
    }
}

/// Returns the path to `config.toml` in the executable's directory.
pub fn conf_path() -> PathBuf {
    env::current_exe().unwrap()
        .parent().unwrap()
        .join("config.toml")
}

/// Returns a subdirectory path relative to the executable's directory.
pub fn exe_path(sub: &str) -> String {
    let base_dir: PathBuf = env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();

    base_dir.join(sub).to_string_lossy().into_owned()
}

/// Generates a default config file.
pub fn generate_config() -> std::io::Result<()> {
    save(&Config::default())
}

/// Saves the given config to `config.toml` in pretty TOML format.
pub fn save(cfg: &Config) -> std::io::Result<()> {
    fs::write(conf_path(), toml::to_string_pretty(cfg).unwrap())
}