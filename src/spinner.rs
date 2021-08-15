use indicatif::{ProgressBar, ProgressStyle};

#[derive(Debug)]
pub struct Spinner {
    pub spin: ProgressBar,
}

impl Default for Spinner {
    fn default() -> Self {
        let spin = ProgressBar::new_spinner();

        spin.set_style(
            ProgressStyle::default_spinner().template("{spinner:.yellow} Searching: {wide_msg}"),
        );
        spin.enable_steady_tick(50);

        Self { spin }
    }
}

impl Spinner {
    pub fn msg(&self, msg: String) {
        self.spin.set_message(msg);
    }

    pub fn set_count_style(&self) {
        self.spin.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.yellow} Calculating total size: {msg} MB"),
        );
        self.spin.set_message("0.0");
    }

    pub fn end(&self) {
        self.spin.finish_and_clear();
    }
}
