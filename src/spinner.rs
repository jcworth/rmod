use core::time;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
};

use indicatif::{ProgressBar, ProgressStyle};

pub struct Spinner {
    spin: ProgressBar
}

impl Spinner {
    pub fn new() -> Self {
        let spin = ProgressBar::new_spinner();

        spin.set_style(ProgressStyle::default_spinner().template("{spinner} Searching..."));
        spin.enable_steady_tick(15);
        Self {spin}
    }

    pub fn end(&self) {
        self.spin.finish_and_clear();
    }
}

