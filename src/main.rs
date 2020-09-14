extern crate ctrlc;

use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{ JoinHandle, spawn };

mod window_structs;
mod windows_specific;

fn set_cmd_interrupt_handler(stop: Arc<AtomicBool>) {
    ctrlc::set_handler(move || {
        println!("Stopping");
        stop.swap(true, Ordering::Relaxed);
    })
    .expect("Error setting interrupt handler");
}

#[cfg(target_os = "windows")]
unsafe fn open_message_box(stop: Arc<AtomicBool>) -> JoinHandle<()> {
    spawn(move || {
        windows_specific::open_message_box();
        stop.swap(true, Ordering::Relaxed);
    })
}

#[cfg(target_os = "windows")]
unsafe fn loop_until_interrupt(stop: Arc<AtomicBool>, titles_to_center_on: HashSet<&str>) {
    windows_specific::loop_until_interrupt(stop, titles_to_center_on);
}

fn main() {
    let stop: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let games: HashSet<&str> = ["Skyrim", "Skyrim Special Edition"]
        .iter()
        .cloned()
        .collect();
    set_cmd_interrupt_handler(stop.clone());
    unsafe {
        let message_box_thread = open_message_box(stop.clone());
        loop_until_interrupt(stop, games);
        message_box_thread
            .join()
            .expect("Thread ended unexpectedly");
    };
}
