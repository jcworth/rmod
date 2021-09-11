use indicatif::{ProgressBar, ProgressStyle};

use crate::FileSize;

#[derive(Debug)]
pub struct Spinner {
    spin: ProgressBar,
}

pub enum SpinStyle {
    Search,
    Count,
    Remove,
}

impl Spinner {
    pub fn new() -> Self {
        let spin = ProgressBar::new_spinner();

        Self { spin }
    }
}

impl Spinner {
    fn spin(&self) -> &ProgressBar {
        &self.spin
    }

    pub fn msg(&self, (name, size): (&str, f64)) {
        self.spin()
            .set_message(format!("{}: {:.2}", name, FileSize::MB.get_value(size)));
    }

    pub fn set_style(&self, style: SpinStyle) {
        let spinner = self.spin();

        let (text, default) = match style {
            SpinStyle::Search => ("Searching: {wide_msg}", "..."),
            SpinStyle::Count => ("Calculating size: {msg} MB", "0.0"),
            // TODO
            SpinStyle::Remove => ("", ""),
        };

        let msg = format!("{{spinner:.yellow}} {}", text);
        spinner.set_style(ProgressStyle::default_spinner().template(&msg));

        spinner.set_message(default);
        spinner.enable_steady_tick(50);
    }

    pub fn end(&self) {
        self.spin().finish_and_clear();
    }
}
