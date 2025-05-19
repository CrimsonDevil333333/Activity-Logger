mod config;
mod platform;
mod tracker;
mod tray;

use std::path::PathBuf;

fn main() {
    println!("Starting Activity Logger...");

    // Load configuration with proper error handling
    let config = match config::Config::from_file("config.json") {
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

    // Compose full paths to log files inside the log_dir
    let key_log_path = log_dir.join(&config.key_log_file);
    let active_log_path = log_dir.join(&config.window_log_file);

    // Path to config file
    let config_path = PathBuf::from("config.json");

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
