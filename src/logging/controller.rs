use super::{log_to_console, FileWriter, Logger, Severity, DEFAULT_LOG_DIR, NAME};
use std::{path::Path, thread::JoinHandle};

/// The creator of logs
pub struct LogController {
    /// The log writer thread
    file_writer: FileWriter,

    /// A handle to join the log writer thread
    join_handle: JoinHandle<()>,
}

/// Creates the directory for logs if it does not already exist
fn create_directory(directory: &Path) {
    if Path::new(directory).try_exists().unwrap_or(false) {
        return;
    }

    if let Err(error) = std::fs::create_dir(directory) {
        log_to_console(
            Severity::Error,
            NAME,
            &format_args!("Failed to create \"{}\" - {}", directory.display(), error),
        )
    }
}

impl LogController {
    /// Creates a new [`LogController`]
    pub fn new(directory: Option<&Path>) -> Self {
        let directory = directory.unwrap_or(Path::new(DEFAULT_LOG_DIR));
        create_directory(directory);

        let (file_writer, join_handle) = FileWriter::new(directory.to_path_buf());

        LogController {
            file_writer,
            join_handle,
        }
    }

    /// Creates a [`Logger`]
    pub fn logger(&self, name: &'static str) -> Logger {
        Logger::new(name, self.file_writer.clone())
    }

    /// Waits for the file writing thread to finish before returning
    pub fn finish(self) {
        drop(self.file_writer);
        self.join_handle.join().unwrap();
    }
}
