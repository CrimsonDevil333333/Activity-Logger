#[cfg(target_os = "linux")]
use std::process::Command;
use crate::tracker;

#[cfg(target_os = "linux")]
pub fn start_service() {
    std::thread::spawn(|| {
        tracker::track_activity(get_active_window);
    })
    .join()
    .unwrap();
}

fn get_active_window() -> Option<String> {
    let output = Command::new("xdotool")
        .arg("getwindowfocus")
        .arg("getwindowname")
        .output()
        .ok()?;
    String::from_utf8(output.stdout).ok().map(|s| s.trim().to_string())
}
