mod tracker;
mod platform;
mod config;

fn main() {
    println!("Starting activity tracker...");
    platform::start_service();
}


// sudo apt install xdotool