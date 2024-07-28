use super::FileWriter;
use alexandria::Severity;
use std::{borrow::Cow, fmt::Display};

#[cfg(debug_assertions)]
use super::log_to_console;

/// Logs messages to a file and to the console
pub struct Logger {
    /// The name of the file to log to
    name: &'static str,

    /// The log writer thread
    file_writer: FileWriter,
}

/// Makes sure a logger name is only lowercase letters, digits, and dashes
fn is_valid_name(name: &str) -> bool {
    if name.len() == 0 {
        return false;
    }

    for c in name.chars() {
        if !c.is_lowercase() && !c.is_digit(10) && c != '-' {
            return false;
        }
    }

    true
}

impl Logger {
    /// Creates a new [`Logger`]
    pub(super) fn new(name: &'static str, file_writer: FileWriter) -> Self {
        assert!(is_valid_name(name));

        file_writer.open(name);

        Logger { name, file_writer }
    }

    /// Logs a message to a file and to the console
    pub fn log<T: Display + ?Sized>(&self, severity: Severity, message: &T) {
        self.log_string(severity, message.to_string());
    }

    /// Logs a message to a file and to the console
    pub fn log_string<T: Into<Cow<'static, str>>>(&self, severity: Severity, message: T) {
        let message = message.into();

        #[cfg(debug_assertions)]
        log_to_console(severity, self.name, &message);

        self.file_writer.log(severity, self.name, message);
    }

    /// Creates a new [`Logger`]
    pub fn new_logger(&self, name: &'static str) -> Logger {
        Logger::new(name, self.file_writer.clone())
    }
}

impl Clone for Logger {
    fn clone(&self) -> Self {
        self.file_writer.open(self.name);

        Logger {
            name: self.name,
            file_writer: self.file_writer.clone(),
        }
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.file_writer.close(self.name);
    }
}
