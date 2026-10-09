use crate::{
    Error, Result, ThreadManager,
    logging::{
        CombinedFileOutput, FormatterKind, HumanReadableFormatter, JsonFormatter, LogController,
        LogMessage, LogOutput, LoggingOptions, ScopeFilesOutput, StdoutOutput,
    },
};
use std::{
    io::{IsTerminal, stdout},
    path::{Path, PathBuf},
    sync::mpsc::Receiver,
};
use time::{DateTime, NoTimeZone};

#[cfg(debug_assertions)]
const MAX_LOG_FOLDERS: usize = 10;

#[cfg(not(debug_assertions))]
const MAX_LOG_FOLDERS: usize = 3;

impl LogController {
    /// Spawn the logger thread
    pub(crate) fn spawn_thread<Game: crate::Game>(
        &self,
        thread_manager: &ThreadManager,
        options: &LoggingOptions<Game>,
    ) -> Result<()> {
        // Build log folder path
        let base_folder = options.log_folder.as_path();
        let now = DateTime::<NoTimeZone>::now_local();
        let log_folder = base_folder.join(format!(
            "{:04}{:02}{:02}-{:02}{:02}{:02}",
            now.year(),
            now.month() + 1,
            now.day(),
            now.hour(),
            now.minute(),
            now.second()
        ));

        // Create log folder and clean it up if needed
        if options.log_combined != FormatterKind::None || options.log_scoped != FormatterKind::None
        {
            std::fs::create_dir_all(&log_folder).map_err(|error| {
                Error::new_with(
                    format!("unable to create \"{}\"", log_folder.display()),
                    error,
                )
            })?;

            cleanup_log_folder(&base_folder).map_err(|error| {
                Error::new_with(
                    format!("unable to cleanup \"{}\"", base_folder.display()),
                    error,
                )
            })?;
        }

        // Create outputs
        let outputs = create_outputs(options, log_folder)?;

        let receiver = self
            .reciever
            .lock()
            .unwrap()
            .take()
            .expect("attempting to start the logging thread more than once");

        let message_queue = self.message_queue.clone();
        thread_manager.spawn(
            "Logging".to_string(),
            move |_| log_thread(receiver, outputs),
            move || {
                message_queue.send(None).ok();
            },
        )
    }
}

/// The main function for the logging thread
fn log_thread(
    messages: Receiver<Option<LogMessage>>,
    mut outputs: Vec<Box<dyn LogOutput>>,
) -> Result<()> {
    while let Ok(Some(message)) = messages.recv() {
        for output in &mut outputs {
            output.output(&message)?;
        }
    }

    Ok(())
}

/// Cleanup old log files in `path`
fn cleanup_log_folder(path: &Path) -> std::io::Result<()> {
    let mut log_folders = Vec::new();
    for entry in std::fs::read_dir(path)? {
        log_folders.push(entry?.file_name());
    }

    if log_folders.len() <= MAX_LOG_FOLDERS {
        return Ok(());
    }

    log_folders.sort();

    for folder in &log_folders[..log_folders.len() - MAX_LOG_FOLDERS] {
        let path = path.join(folder);
        std::fs::remove_dir_all(path)?;
    }

    Ok(())
}

/// Create the log outputs based on the options
fn create_outputs<Game: crate::Game>(
    options: &LoggingOptions<Game>,
    log_folder: PathBuf,
) -> Result<Vec<Box<dyn LogOutput>>> {
    let mut outputs: Vec<Box<dyn LogOutput>> = Vec::new();
    match options.log_stdout {
        FormatterKind::None => {}
        FormatterKind::Human => outputs.push(Box::new(StdoutOutput::new(
            HumanReadableFormatter::new(stdout().is_terminal()),
        )?)),
        FormatterKind::Json => {
            outputs.push(Box::new(StdoutOutput::new(JsonFormatter::new(false))?))
        }
        FormatterKind::JsonPretty => {
            outputs.push(Box::new(StdoutOutput::new(JsonFormatter::new(true))?))
        }
    }
    match options.log_combined {
        FormatterKind::None => {}
        FormatterKind::Human => outputs.push(Box::new(CombinedFileOutput::new(
            &log_folder,
            HumanReadableFormatter::new(false),
        )?)),
        FormatterKind::Json => outputs.push(Box::new(CombinedFileOutput::new(
            &log_folder,
            JsonFormatter::new(false),
        )?)),
        FormatterKind::JsonPretty => outputs.push(Box::new(CombinedFileOutput::new(
            &log_folder,
            JsonFormatter::new(true),
        )?)),
    }
    match options.log_scoped {
        FormatterKind::None => {}
        FormatterKind::Human => outputs.push(Box::new(ScopeFilesOutput::new(
            log_folder,
            HumanReadableFormatter::new(false),
        ))),
        FormatterKind::Json => outputs.push(Box::new(ScopeFilesOutput::new(
            log_folder,
            JsonFormatter::new(false),
        ))),
        FormatterKind::JsonPretty => outputs.push(Box::new(ScopeFilesOutput::new(
            log_folder,
            JsonFormatter::new(true),
        ))),
    }

    Ok(outputs)
}
