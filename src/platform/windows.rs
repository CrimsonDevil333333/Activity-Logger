use crate::tracker;
#[cfg(target_os = "windows")]
#[cfg(target_os = "windows")]
pub fn start_service() {
    std::thread::spawn(|| {
        tracker::track_activity(get_active_window);
    })
    .join()
    .unwrap();
}

#[cfg(target_os = "windows")]
fn get_active_window() -> Option<String> {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use std::ptr::null_mut;
    use winapi::um::processthreadsapi::OpenProcess;
    use winapi::um::psapi::{EnumProcessModules, GetModuleBaseNameW};
    use winapi::um::winnt::PROCESS_QUERY_INFORMATION;
    use winapi::um::winnt::PROCESS_VM_READ;
    use winapi::um::winuser::{
        GetForegroundWindow, GetWindowTextLengthW, GetWindowTextW, GetWindowThreadProcessId,
    };

    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.is_null() {
            return None;
        }

        // Get window title
        let len = GetWindowTextLengthW(hwnd) + 1;
        let mut title = String::new();
        if len > 1 {
            let mut buffer = vec![0u16; len as usize];
            if GetWindowTextW(hwnd, buffer.as_mut_ptr(), len) > 0 {
                title = OsString::from_wide(&buffer[..(len - 1) as usize])
                    .to_string_lossy()
                    .trim_matches(|c: char| !c.is_ascii_graphic() && !c.is_whitespace())
                    .to_string();
            }
        }

        // Get process ID
        let mut pid = 0;
        GetWindowThreadProcessId(hwnd, &mut pid);

        // Open process
        let h_process = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, pid);
        let mut exe_name = String::from("Unknown");

        if !h_process.is_null() {
            let mut h_mod = null_mut();
            let mut cb_needed = 0;

            if EnumProcessModules(
                h_process,
                &mut h_mod,
                std::mem::size_of::<usize>() as u32,
                &mut cb_needed,
            ) != 0
            {
                let mut exe_buffer = [0u16; 260];
                if GetModuleBaseNameW(h_process, h_mod, exe_buffer.as_mut_ptr(), 260 as u32) > 0 {
                    exe_name = OsString::from_wide(&exe_buffer)
                        .to_string_lossy()
                        .trim_matches(|c: char| !c.is_ascii_graphic() && !c.is_whitespace())
                        .to_string();
                }
            }
        }

        // Ignore blank or unhelpful titles
        if title.is_empty() || title == exe_name {
            return None;
        }

        Some(format!("App: {} | Title: {}", exe_name, title))
    }
}
