use std::{fmt::Display, time::Duration};

use indicatif::{ProgressBar, ProgressStyle};

use crate::theme::THEME;

/// A spinner that renders progress indication.
///
/// Implemented via theming of [`indicatif::ProgressBar`](https://docs.rs/indicatif).
pub struct Spinner {
    spinner: ProgressBar,
}

impl Default for Spinner {
    fn default() -> Self {
        let spinner = ProgressBar::new_spinner();
        spinner.enable_steady_tick(Duration::from_millis(100));
        Self { spinner }
    }
}

impl Spinner {
    /// Starts the spinner.
    pub fn start(&mut self, message: impl Display) {
        let theme = THEME.lock().unwrap();

        self.spinner.set_style(
            ProgressStyle::with_template(&theme.format_spinner_start())
                .unwrap()
                .tick_chars(&theme.spinner_chars()),
        );

        self.spinner.set_message(message.to_string());
    }

    /// Stops the spinner.
    pub fn stop(&mut self, message: impl Display) {
        let theme = THEME.lock().unwrap();

        // Workaround: the next line doesn't "jump" around while resizing the terminal.
        self.spinner
            .println(theme.format_spinner_stop(&message.to_string()));
        self.spinner.finish_and_clear();
    }
}
