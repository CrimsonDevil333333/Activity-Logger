use crate::config::Config;

// Add notify-rust = "4.10.0" to Cargo.toml for cross-platform notifications
#[cfg(target_os = "windows")]
use notify_rust::Notification;

pub enum NotificationType {
    Start,
    Stop,
    Error(String),
}

pub struct Notifier<'a> {
    config: &'a Config,
}

impl<'a> Notifier<'a> {
    pub fn new(config: &'a Config) -> Self {
        Notifier { config }
    }

    pub fn notify(&self, notification_type: NotificationType) {
        match notification_type {
            NotificationType::Start => {
                if self.config.notify_on_start() {
                    show_notification("Activity Logger", "Logging started", None);
                }
            }
            NotificationType::Stop => {
                if self.config.notify_on_stop() {
                    show_notification("Activity Logger", "Logging stopped", None);
                }
            }
            NotificationType::Error(ref msg) => {
                if self.config.notify_on_error() {
                    show_notification("Activity Logger Error", msg, Some("error"));
                }
            }
        }
    }
}

fn show_notification(summary: &str, body: &str, hint: Option<&str>) {
    #[cfg(target_os = "windows")]
    {
        let mut notif = Notification::new();
        notif.summary(summary).body(body);
        if let Some(_hint) = hint {
            // The 'hint' method is not available in notify-rust; you may set other properties if needed.
            // For example, you could set an icon or appname here if desired.
        }
        let _ = notif.show();
    }
    #[cfg(not(target_os = "windows"))]
    {
        // For other platforms, you can add similar notification logic if needed
        println!("[{}] {}", summary, body);
    }
}