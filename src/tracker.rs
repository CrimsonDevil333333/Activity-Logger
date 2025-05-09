use chrono::Local;
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::{fs::OpenOptions, io::Write, thread, time::Duration};

pub fn track_activity<F>(get_active_window: F)
where
    F: Fn() -> Option<String> + Send + 'static,
{
    let device_state = DeviceState::new();
    let mut last_keys = vec![];
    let mut last_window = String::new();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/activity_log.txt")
        .expect("Could not open log file");

    loop {
        let keys = device_state.get_keys();
        if keys != last_keys {
            last_keys = keys.clone();
            let keys_str = keys
                .iter()
                .map(|k| format!("{:?}", k))
                .collect::<Vec<_>>()
                .join(", ");
            writeln!(
                file,
                "[{}] Keys: {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                keys_str
            )
            .unwrap();
        }

        if let Some(title) = get_active_window() {
            if title != last_window {
                last_window = title.clone();
                writeln!(
                    file,
                    "[{}] Active Window: {}",
                    Local::now().format("%Y-%m-%d %H:%M:%S"),
                    title
                )
                .unwrap();
            }
        }

        thread::sleep(Duration::from_millis(500));
    }
}
