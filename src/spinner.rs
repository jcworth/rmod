use core::time;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
};

pub fn create_spinner(is_running: Arc<AtomicBool>) -> JoinHandle<()> {
    let spinner_handle = thread::spawn(move || {
        let status_chars = vec!['|', '/', '-', '\\'];
        let mut curr = 0;

        while is_running.load(Ordering::Relaxed) {
            // Clear stderr
            eprintln!("{}[2J", 27 as char);

            if curr == 4 {
                curr = 0
            };

            // print to stderr at position 1
            eprintln!("{}[;H{}", 27 as char, status_chars[curr]);
            curr += 1;

            thread::sleep(time::Duration::from_millis(100));
        }
        // Clear stderr at end of loop
        eprintln!("{}[2J", 27 as char);
    });
    spinner_handle
}
