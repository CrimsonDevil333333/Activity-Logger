#[cfg(target_os = "windows")]
use winapi::um::winuser::{GetForegroundWindow, GetWindowTextLengthW, GetWindowTextW};
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use crate::tracker;

#[cfg(target_os = "windows")]
pub fn start_service() {
    std::thread::spawn(|| {
        tracker::track_activity(get_active_window);
    })
    .join()
    .unwrap();
}

fn get_active_window() -> Option<String> {
    unsafe {
        let hwnd = GetForegroundWindow();
        let len = GetWindowTextLengthW(hwnd) + 1;
        if len == 1 {
            return None;
        }
        let mut buffer = vec![0u16; len as usize];
        GetWindowTextW(hwnd, buffer.as_mut_ptr(), len);
        let os_string = OsString::from_wide(&buffer[..(len - 1) as usize]);
        Some(os_string.to_string_lossy().into_owned())
    }
}
