#[cfg(target_os = "macos")]
use cocoa::appkit::NSWorkspace;
use cocoa::base::{nil, id};
use crate::tracker;

#[cfg(target_os = "macos")]
pub fn start_service() {
    std::thread::spawn(|| {
        tracker::track_activity(get_active_window);
    })
    .join()
    .unwrap();
}

fn get_active_window() -> Option<String> {
    unsafe {
        let workspace: id = NSWorkspace::sharedWorkspace(nil);
        let app = workspace.frontmostApplication();
        let name = app.localizedName();
        Some(name.to_string())
    }
}
