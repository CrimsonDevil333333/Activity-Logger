use std::{fs, path::PathBuf, process, sync::mpsc};

use tray_item::{IconSource, TrayItem};

pub enum TrayMessage {
    Quit,
    ShowLogs,
    OpenConfig,
}

#[cfg(target_os = "windows")]
fn get_icon_source(active: bool) -> IconSource {
    if active {
        IconSource::Resource("active-icon")
    } else {
        IconSource::Resource("default-icon")
    }
}

#[cfg(not(target_os = "windows"))]
fn get_icon_source(active: bool) -> IconSource {
    let icon_data: Vec<u8> = if active {
        include_bytes!("../assets/active_icon.png").to_vec()
    } else {
        include_bytes!("../assets/icon.png").to_vec()
    };

    IconSource::Data {
        data: icon_data,
        width: 32,
        height: 32,
    }
}

fn clear_logs(key_log_path: &PathBuf, active_log_path: &PathBuf) -> Result<(), std::io::Error> {
    std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(key_log_path)?;

    std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(active_log_path)?;

    Ok(())
}

fn clear_screenshots(screenshot_dir: &PathBuf) -> Result<(), std::io::Error> {
    if screenshot_dir.exists() && screenshot_dir.is_dir() {
        for entry in fs::read_dir(screenshot_dir)? {
            let entry = entry?;
            if entry.path().is_file() {
                fs::remove_file(entry.path())?;
            }
        }
    }
    Ok(())
}

pub fn create_tray_icon(
    log_dir: PathBuf,
    config_path: PathBuf,
    key_log_path: PathBuf,
    active_log_path: PathBuf,
) -> Result<(), String> {
    let (sender, receiver) = mpsc::channel();
    let thread_sender = sender.clone();

    let screenshot_dir = log_dir.join("screenshots");

    std::thread::spawn(move || {
        let mut tray = match TrayItem::new("Activity Logger", get_icon_source(false)) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Failed to create tray item: {}", e);
                let _ = thread_sender.send(TrayMessage::Quit);
                return;
            }
        };

        tray.inner_mut().add_label("Activity Logger").ok();

        // Show Logs
        tray.add_menu_item("Show Logs", {
            let log_dir = log_dir.clone();
            let sender = sender.clone();
            move || {
                if let Err(e) = open::that(&log_dir) {
                    eprintln!("Failed to open logs: {}", e);
                }
                let _ = sender.send(TrayMessage::ShowLogs);
            }
        })
        .ok();

        // Open Config
        tray.add_menu_item("Open Config", {
            let config_path = config_path.clone();
            let sender = sender.clone();
            move || {
                if let Err(e) = open::that(&config_path) {
                    eprintln!("Failed to open config: {}", e);
                }
                let _ = sender.send(TrayMessage::OpenConfig);
            }
        })
        .ok();

        // Open Screenshots Folder
        tray.add_menu_item("Open Screenshots Folder", {
            let screenshot_dir = screenshot_dir.clone();
            move || {
                if let Err(e) = open::that(&screenshot_dir) {
                    eprintln!("Failed to open screenshots folder: {}", e);
                }
            }
        })
        .ok();

        // Clear Logs
        tray.add_menu_item("Clear Logs", {
            let key_log_path = key_log_path.clone();
            let active_log_path = active_log_path.clone();
            move || match clear_logs(&key_log_path, &active_log_path) {
                Ok(_) => println!("✅ Logs cleared."),
                Err(e) => eprintln!("❌ Failed to clear logs: {}", e),
            }
        })
        .ok();

        // Clear Screenshots
        tray.add_menu_item("Clear Screenshots", {
            let screenshot_dir = screenshot_dir.clone();
            move || match clear_screenshots(&screenshot_dir) {
                Ok(_) => println!("✅ Screenshots cleared."),
                Err(e) => eprintln!("❌ Failed to clear screenshots: {}", e),
            }
        })
        .ok();

        // Clear Everything
        tray.add_menu_item("Clear Everything", {
            let screenshot_dir = screenshot_dir.clone();
            let key_log_path = key_log_path.clone();
            let active_log_path = active_log_path.clone();
            move || {
                let log_result = clear_logs(&key_log_path, &active_log_path);
                let ss_result = clear_screenshots(&screenshot_dir);

                match (log_result, ss_result) {
                    (Ok(_), Ok(_)) => println!("✅ All logs and screenshots cleared."),
                    (Err(e), _) | (_, Err(e)) => eprintln!("❌ Failed to clear everything: {}", e),
                }
            }
        })
        .ok();

        tray.inner_mut().add_separator().ok();

        // Quit
        tray.add_menu_item("Quit", move || {
            let _ = sender.send(TrayMessage::Quit);
        })
        .ok();

        tray.set_icon(get_icon_source(true)).ok();

        while let Ok(msg) = receiver.recv() {
            if let TrayMessage::Quit = msg {
                process::exit(0);
            }
        }
    });

    Ok(())
}
