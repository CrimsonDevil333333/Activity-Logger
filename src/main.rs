#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod config;
mod embedded; // new module
mod platform;
mod screenshot_mod;
mod tracker;
mod tray;
use std::path::PathBuf;

use screenshot_mod::start_screenshot_loop;

fn main() {
    println!("Starting Activity Logger...");

    // Define where config should be extracted (e.g. current dir or temp)
    let config_path = PathBuf::from("config.json");

    // Extract embedded config.json if it does NOT exist
    if !config_path.exists() {
        if let Err(e) = embedded::extract_asset_to_file("config.json", &config_path) {
            eprintln!("Failed to extract embedded config.json: {}", e);
            return;
        }
    }

    // Now load config as before
    let config = match config::Config::from_file(config_path.to_str().unwrap()) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            return;
        }
    };

    // Determine log directory path (includes "activity_logger" if temp)
    let log_dir = config.log_directory_path();

    // Create required directories if they don't exist
    if let Err(e) = std::fs::create_dir_all(&log_dir) {
        eprintln!("Failed to create log directory: {}", e);
        return;
    }

    // Start screenshot capture if enabled
    if config.screenshot_enabled.unwrap_or(false) {
        let stop_flag = config.screenshot_enabled.unwrap_or(true);
        let interval_secs = config.screenshot_interval_secs.unwrap_or(30);
        let resolution = config.screenshot_resolution;

        start_screenshot_loop(&log_dir, interval_secs, !stop_flag.clone(), resolution);
    }

    // Compose full paths to log files inside the log_dir
    let key_log_path = log_dir.join(&config.key_log_file);
    let active_log_path = log_dir.join(&config.window_log_file);

    // Initialize tray icon with all required paths
    match tray::create_tray_icon(log_dir.clone(), config_path, key_log_path, active_log_path) {
        Ok(_) => {
            println!("Activity Logger started successfully");
            println!("Logs will be saved to: {}", log_dir.display());

            // Start the platform-specific service
            platform::start_service();
        }
        Err(e) => {
            eprintln!("Failed to initialize tray icon: {}", e);
        }
    }
}
