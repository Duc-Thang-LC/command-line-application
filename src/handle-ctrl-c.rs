use std::{thread, time::Duration};
use ctrlc;

fn main() {
    // Handle control + C signal
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
    })
    .expect("Error setting Ctrl-C handler");

    // Wait for 2 seconds
    thread::sleep(Duration::from_secs(2));
}