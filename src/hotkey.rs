use crate::config::Config;
use std::sync::{Arc, Mutex};
use std::thread;

// You may need to add rdev = "0.5" to your Cargo.toml
use rdev::{listen, Event, EventType, Key};

pub enum HotkeyAction {
    PauseResume,
    Screenshot,
}

pub struct HotkeyHandler {
    pub pause_resume_combo: Option<Vec<Key>>,
    pub screenshot_combo: Option<Vec<Key>>,
    pub action_callback: Arc<dyn Fn(HotkeyAction) + Send + Sync>,
}

impl HotkeyHandler {
    pub fn from_config(config: &Config, action_callback: Arc<dyn Fn(HotkeyAction) + Send + Sync>) -> Self {
        let pause_resume_combo = config.hotkey_pause_resume().map(parse_hotkey_string);
        let screenshot_combo = config.hotkey_screenshot().map(parse_hotkey_string);
        HotkeyHandler {
            pause_resume_combo,
            screenshot_combo,
            action_callback,
        }
    }

    pub fn start_listening(self) {
        let pause_resume_combo = self.pause_resume_combo.clone();
        let screenshot_combo = self.screenshot_combo.clone();
        let action_callback = self.action_callback.clone();

        thread::spawn(move || {
            let pressed_keys = Arc::new(Mutex::new(Vec::new()));
            let pressed_keys_clone = pressed_keys.clone();

            let callback = move |event: Event| {
                match event.event_type {
                    EventType::KeyPress(key) => {
                        let mut keys = pressed_keys_clone.lock().unwrap();
                        if !keys.contains(&key) {
                            keys.push(key);
                        }
                        if let Some(ref combo) = pause_resume_combo {
                            if is_combo_pressed(&keys, combo) {
                                (action_callback)(HotkeyAction::PauseResume);
                            }
                        }
                        if let Some(ref combo) = screenshot_combo {
                            if is_combo_pressed(&keys, combo) {
                                (action_callback)(HotkeyAction::Screenshot);
                            }
                        }
                    }
                    EventType::KeyRelease(key) => {
                        let mut keys = pressed_keys_clone.lock().unwrap();
                        keys.retain(|&k| k != key);
                    }
                    _ => {}
                }
            };

            // Listen will block, so run in a thread
            if let Err(e) = listen(callback) {
                eprintln!("Hotkey listener error: {:?}", e);
            }
        });
    }
}

/// Parse a hotkey string like "Ctrl+Shift+P" into a Vec<Key>
fn parse_hotkey_string(combo: &str) -> Vec<Key> {
    combo
        .split('+')
        .filter_map(|part| match part.trim().to_lowercase().as_str() {
            "ctrl" => Some(Key::ControlLeft),
            "shift" => Some(Key::ShiftLeft),
            "alt" => Some(Key::Alt),
            "cmd" | "win" => Some(Key::MetaLeft),
            "a" => Some(Key::KeyA),
            "b" => Some(Key::KeyB),
            "c" => Some(Key::KeyC),
            "d" => Some(Key::KeyD),
            "e" => Some(Key::KeyE),
            "f" => Some(Key::KeyF),
            "g" => Some(Key::KeyG),
            "h" => Some(Key::KeyH),
            "i" => Some(Key::KeyI),
            "j" => Some(Key::KeyJ),
            "k" => Some(Key::KeyK),
            "l" => Some(Key::KeyL),
            "m" => Some(Key::KeyM),
            "n" => Some(Key::KeyN),
            "o" => Some(Key::KeyO),
            "p" => Some(Key::KeyP),
            "q" => Some(Key::KeyQ),
            "r" => Some(Key::KeyR),
            "s" => Some(Key::KeyS),
            "t" => Some(Key::KeyT),
            "u" => Some(Key::KeyU),
            "v" => Some(Key::KeyV),
            "w" => Some(Key::KeyW),
            "x" => Some(Key::KeyX),
            "y" => Some(Key::KeyY),
            "z" => Some(Key::KeyZ),
            _ => None,
        })
        .collect()
}

/// Check if all keys in combo are pressed
fn is_combo_pressed(pressed: &[Key], combo: &[Key]) -> bool {
    combo.iter().all(|k| pressed.contains(k))
}