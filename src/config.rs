use base64::{engine::general_purpose, Engine as _};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{fs, io, path::PathBuf};

/// One host entry
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entry {
    pub name: String,
    pub ip: String,
    pub username: String,
    pub password: Option<String>, // base64‐encoded
    pub rsa_key: Option<String>,  // base64‐encoded private key text
    pub auth_mode: AuthMode,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AuthMode {
    Password,
    Rsa,
    Both,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub entries: Vec<Entry>,
}

impl Config {
    pub fn find(&self, name: &str) -> Option<&Entry> {
        self.entries.iter().find(|e| e.name == name)
    }
    pub fn find_mut(&mut self, name: &str) -> Option<&mut Entry> {
        self.entries.iter_mut().find(|e| e.name == name)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("I/O: {0}")]
    Io(#[from] io::Error),
    #[error("JSON: {0}")]
    Json(#[from] serde_json::Error),
}

pub struct ConfigManager {
    path: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Result<Self, ConfigError> {
        let pd = ProjectDirs::from("com", "example", "ssh_cli")
            .expect("Cannot determine config directory");
        let path = pd.config_dir().join("config.json");
        fs::create_dir_all(path.parent().unwrap())?;
        Ok(Self { path })
    }

    pub fn load(&self) -> Result<Config, ConfigError> {
        if self.path.exists() {
            let data = fs::read_to_string(&self.path)?;
            Ok(serde_json::from_str(&data)?)
        } else {
            Ok(Config::default())
        }
    }

    pub fn save(&self, cfg: &Config) -> Result<(), ConfigError> {
        let json = serde_json::to_string_pretty(cfg)?;
        fs::write(&self.path, json)?;
        Ok(())
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

/// Helpers
pub fn encode_clear(text: &str) -> String {
    general_purpose::STANDARD.encode(text)
}
pub fn decode_clear(enc: &str) -> Result<String, base64::DecodeError> {
    let bytes = general_purpose::STANDARD.decode(enc)?;
    Ok(String::from_utf8_lossy(&bytes).to_string())
}
