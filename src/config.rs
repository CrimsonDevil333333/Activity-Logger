use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub key_log_file: String,
    pub window_log_file: String,
    pub log_dir: String,
    pub inactivity_timeout_secs: u64,
}

impl Config {
    pub fn from_file(path: &str) -> Self {
        let config_data = fs::read_to_string(path).expect("Unable to read config file");
        serde_json::from_str(&config_data).expect("Invalid config format")
    }

    pub fn full_key_log_path(&self) -> std::path::PathBuf {
        let dir = if self.log_dir.to_lowercase() == "temp" {
            std::env::temp_dir()
        } else {
            std::path::PathBuf::from(&self.log_dir)
        };

        dir.join(&self.key_log_file)
    }

    pub fn active_window_log_path(&self) -> std::path::PathBuf {
        let dir = if self.log_dir.to_lowercase() == "temp" {
            std::env::temp_dir()
        } else {
            std::path::PathBuf::from(&self.log_dir)
        };

        dir.join(&self.window_log_file)
    }

    pub fn timeout_secs(&self) -> u64 {
        self.inactivity_timeout_secs
    }
}
