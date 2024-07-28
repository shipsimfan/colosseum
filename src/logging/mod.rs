//! Utilities for logging messages

use std::{fmt::Display, io::Write};

mod controller;
mod file_writer;
mod logger;
mod macros;

pub use alexandria::Severity;
pub use controller::LogController;
pub use logger::Logger;

use file_writer::FileWriter;

const DEFAULT_LOG_DIR: &str = "logs/";
const NAME: &str = "Logging";

/// Logs a given message to the console
fn log_to_console<T: Display>(severity: Severity, name: &'static str, message: &T) {
    let mut stderr = std::io::stderr().lock();

    writeln!(
        stderr,
        "[\x1B[1;{}m{}\x1B[0m][{}] {}",
        severity_color(severity),
        severity,
        name,
        message
    )
    .ok();
}

fn severity_color(severity: Severity) -> &'static str {
    match severity {
        Severity::Debug => "37",
        Severity::Info => "36",
        Severity::Warning => "33",
        Severity::Error | Severity::Fatal => "31",
    }
}
