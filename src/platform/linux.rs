use crate::tracker;
#[cfg(target_os = "linux")]
use std::process::Command;

#[cfg(target_os = "linux")]
pub fn start_service() {
    std::thread::spawn(|| {
        tracker::track_activity(get_active_window);
    })
    .join()
    .unwrap();
}

fn get_active_window() -> Option<String> {
    let window_title = Command::new("xdotool")
        .arg("getwindowfocus")
        .arg("getwindowname")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())?
        .trim()
        .to_string();

    let window_class = Command::new("xdotool")
        .arg("getwindowfocus")
        .arg("getwindowclassname")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())?
        .trim()
        .to_string();

    Some(format!("App: {} | Title: {}", window_class, window_title))
}
