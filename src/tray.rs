use std::{path::PathBuf, process, sync::mpsc};
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
    let icon_data = if active {
        include_bytes!("../assets/active_icon.png")
    } else {
        include_bytes!("../assets/icon.png")
    };

    IconSource::Data {
        data: icon_data,
        width: 32,
        height: 32,
    }
}

// Function to truncate/clear log files safely
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

pub fn create_tray_icon(
    log_dir: PathBuf,
    config_path: PathBuf,
    key_log_path: PathBuf,
    active_log_path: PathBuf,
) -> Result<(), String> {
    let (sender, receiver) = mpsc::channel();
    let thread_sender = sender.clone();

    std::thread::spawn(move || {
        let mut tray = match TrayItem::new("Activity Logger", get_icon_source(false)) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Failed to create tray item: {}", e);
                let _ = thread_sender.send(TrayMessage::Quit);
                return;
            }
        };

        if let Err(e) = tray.inner_mut().add_label("Activity Logger") {
            eprintln!("Failed to add label: {}", e);
        }

        if let Err(e) = tray.add_menu_item("Show Logs", {
            let log_dir = log_dir.clone();
            let sender = sender.clone();
            move || {
                if let Err(e) = open::that(&log_dir) {
                    eprintln!("Failed to open logs: {}", e);
                }
                let _ = sender.send(TrayMessage::ShowLogs);
            }
        }) {
            eprintln!("Failed to add Show Logs menu: {}", e);
        }

        if let Err(e) = tray.add_menu_item("Open Config", {
            let config_path = config_path.clone();
            let sender = sender.clone();
            move || {
                if let Err(e) = open::that(&config_path) {
                    eprintln!("Failed to open config: {}", e);
                }
                let _ = sender.send(TrayMessage::OpenConfig);
            }
        }) {
            eprintln!("Failed to add Open Config menu: {}", e);
        }

        // Clear Logs menu item
        if let Err(e) = tray.add_menu_item("Clear Logs", {
            let key_log_path = key_log_path.clone();
            let active_log_path = active_log_path.clone();
            move || match clear_logs(&key_log_path, &active_log_path) {
                Ok(_) => println!("Logs cleared successfully"),
                Err(e) => eprintln!("Failed to clear logs: {}", e),
            }
        }) {
            eprintln!("Failed to add Clear Logs menu: {}", e);
        }

        if let Err(e) = tray.inner_mut().add_separator() {
            eprintln!("Failed to add separator: {}", e);
        }

        if let Err(e) = tray.add_menu_item("Quit", move || {
            let _ = sender.send(TrayMessage::Quit);
        }) {
            eprintln!("Failed to add Quit menu: {}", e);
        }

        if let Err(e) = tray.set_icon(get_icon_source(true)) {
            eprintln!("Failed to set active icon: {}", e);
        }

        while let Ok(msg) = receiver.recv() {
            match msg {
                TrayMessage::Quit => process::exit(0),
                _ => {}
            }
        }
    });

    Ok(())
}
