#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod config;
mod embedded;
mod platform;
mod screenshot_mod;
mod tracker;
mod tray;
mod hotkey;
mod notification;
mod report;

use std::path::PathBuf;
use screenshot_mod::start_screenshot_loop;
use hotkey::{HotkeyAction, HotkeyHandler};
use notification::{Notifier, NotificationType};
use report::SummaryReporter;

fn main() {
    println!("Starting Activity Logger...");

    let config_path = PathBuf::from("config.json");

    if !config_path.exists() {
        if let Err(e) = embedded::extract_asset_to_file("config.json", &config_path) {
            eprintln!("Failed to extract embedded config.json: {}", e);
            return;
        }
    }

    // Load config as a normal struct, no Arc or clone
    let config = match config::Config::from_file(config_path.to_str().unwrap()) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            return;
        }
    };

    let log_dir = config.log_directory_path();

    if let Err(e) = std::fs::create_dir_all(&log_dir) {
        eprintln!("Failed to create log directory: {}", e);
        return;
    }

    use std::sync::Arc;
    let config = Arc::new(config);

    // --- Notification System ---
    let notifier = Notifier::new(config.as_ref());
    notifier.notify(NotificationType::Start);

    // --- Screenshot Capture ---
    if config.screenshot_enabled.unwrap_or(false) {
        let stop_flag = config.screenshot_enabled.unwrap_or(true);
        let interval_secs = config.screenshot_interval_secs.unwrap_or(30);
        let resolution = config.screenshot_resolution;
        start_screenshot_loop(&log_dir, interval_secs, !stop_flag, resolution);
    }

    // --- Hotkey Handler ---
    let hotkey_callback = {
        let config = Arc::clone(&config);
        move |action: HotkeyAction| {
            let notifier = Notifier::new(config.as_ref());
            match action {
                HotkeyAction::PauseResume => {
                    println!("Pause/Resume hotkey pressed.");
                    notifier.notify(NotificationType::Stop);
                }
                HotkeyAction::Screenshot => {
                    println!("Screenshot hotkey pressed.");
                    notifier.notify(NotificationType::Start);
                }
            }
        }
    };
    let hotkey_handler = HotkeyHandler::from_config(&config, Arc::new(hotkey_callback));
    hotkey_handler.start_listening();

    // --- Activity Summary Report ---
    let summary_reporter = SummaryReporter::new(&config);
    if config.summary_report_enabled() {
        if let Err(e) = summary_reporter.generate_report() {
            notifier.notify(NotificationType::Error(format!("Failed to generate summary report: {}", e)));
        }
    }

    let key_log_path = log_dir.join(&config.key_log_file);
    let active_log_path = log_dir.join(&config.window_log_file);

    match tray::create_tray_icon(log_dir.clone(), config_path, key_log_path, active_log_path) {
        Ok(_) => {
            println!("Activity Logger started successfully");
            println!("Logs will be saved to: {}", log_dir.display());
            platform::start_service();
        }
        Err(e) => {
            eprintln!("Failed to initialize tray icon: {}", e);
        }
    }
}
