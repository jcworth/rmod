use indicatif::{ProgressBar, ProgressStyle};

pub struct Spinner {
    spin: ProgressBar
}

impl Default for Spinner {
    fn default() -> Self {
        let spin = ProgressBar::new_spinner();

        spin.set_style(ProgressStyle::default_spinner().template("{spinner} Searching..."));
        spin.enable_steady_tick(15);
        Self {spin}
    }
}

impl Spinner {
    pub fn end(&self) {
        self.spin.finish_and_clear();
    }
}

