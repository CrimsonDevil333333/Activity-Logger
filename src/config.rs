use serde::Deserialize;
use std::{fs, io, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub key_log_file: String,
    pub window_log_file: String,
    pub log_dir: String,
    pub inactivity_timeout_secs: u64,
    pub screenshot_enabled: Option<bool>,
    pub screenshot_interval_secs: Option<u64>,
    pub screenshot_resolution: Option<(u32, u32)>,
    pub hotkeys: Option<HotkeyConfig>,
    pub notification: Option<NotificationConfig>,
    pub summary_report: Option<SummaryReportConfig>,
}

#[derive(Debug, Deserialize)]
pub struct HotkeyConfig {
    pub pause_resume: Option<String>, // e.g., "Ctrl+Shift+P"
    pub screenshot: Option<String>,   // e.g., "Ctrl+Shift+S"
}

#[derive(Debug, Deserialize)]
pub struct NotificationConfig {
    pub on_start: Option<bool>,
    pub on_stop: Option<bool>,
    pub on_error: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct SummaryReportConfig {
    pub enabled: Option<bool>,
    pub interval_days: Option<u32>, // e.g., 1 for daily, 7 for weekly
    pub output_file: Option<String>,
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
        self.resolved_log_dir()
    }

    pub fn timeout_secs(&self) -> u64 {
        self.inactivity_timeout_secs
    }

    pub fn ss_capture_flag(&self) -> bool {
        self.screenshot_enabled.unwrap_or(true)
    }

    pub fn screen_capture_resolution(&self) -> Option<(u32, u32)> {
        self.screenshot_resolution
    }

    pub fn hotkey_pause_resume(&self) -> Option<&str> {
        self.hotkeys.as_ref()?.pause_resume.as_deref()
    }

    pub fn hotkey_screenshot(&self) -> Option<&str> {
        self.hotkeys.as_ref()?.screenshot.as_deref()
    }

    pub fn notify_on_start(&self) -> bool {
        self.notification.as_ref().and_then(|n| n.on_start).unwrap_or(false)
    }

    pub fn notify_on_stop(&self) -> bool {
        self.notification.as_ref().and_then(|n| n.on_stop).unwrap_or(false)
    }

    pub fn notify_on_error(&self) -> bool {
        self.notification.as_ref().and_then(|n| n.on_error).unwrap_or(true)
    }

    pub fn summary_report_enabled(&self) -> bool {
        self.summary_report.as_ref().and_then(|s| s.enabled).unwrap_or(false)
    }

    pub fn summary_report_interval_days(&self) -> u32 {
        self.summary_report.as_ref().and_then(|s| s.interval_days).unwrap_or(1)
    }

    pub fn summary_report_output_file(&self) -> Option<&str> {
        self.summary_report.as_ref()?.output_file.as_deref()
    }
}
