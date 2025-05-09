mod tracker;
mod platform;

fn main() {
    println!("Starting activity tracker...");
    platform::start_service();
}


// sudo apt install xdotool