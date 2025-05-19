use serde::Deserialize;
use std::{fs, io, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub key_log_file: String,
    pub window_log_file: String,
    pub log_dir: String,
    pub inactivity_timeout_secs: u64,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, io::Error> {
        let config_data = fs::read_to_string(path)?;
        serde_json::from_str(&config_data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    /// Returns the base log directory, creating `temp/activity_logger` if needed.
    fn resolved_log_dir(&self) -> PathBuf {
        if self.log_dir.to_lowercase() == "temp" {
            std::env::temp_dir().join("activity_logger")
        } else {
            PathBuf::from(&self.log_dir)
        }
    }

    /// Returns the full key log file path.
    pub fn full_key_log_path(&self) -> PathBuf {
        self.resolved_log_dir().join(&self.key_log_file)
    }

    /// Returns the full active window log file path.
    pub fn active_window_log_path(&self) -> PathBuf {
        self.resolved_log_dir().join(&self.window_log_file)
    }

    /// Returns the log directory path.
    pub fn log_directory_path(&self) -> std::path::PathBuf {
        if self.log_dir.to_lowercase() == "temp" {
            std::env::temp_dir().join("activity_logger")
        } else {
            std::path::PathBuf::from(&self.log_dir)
        }
    }

    pub fn timeout_secs(&self) -> u64 {
        self.inactivity_timeout_secs
    }
}
