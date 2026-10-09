use crate::{
    GameMetadata, LogController, LoggingOptions, Result,
    logging::{
        CombinedFileOutput, FormatterKind, HumanReadableFormatter, JsonFormatter, LogOutput,
        ScopeFilesOutput, StdoutOutput,
    },
    new_error,
};
use std::{
    io::{IsTerminal, stdout},
    path::Path,
    sync::{Arc, Mutex, atomic::AtomicU64},
    time::Instant,
};
use time::{DateTime, NoTimeZone};

#[cfg(debug_assertions)]
const MAX_LOG_FOLDERS: usize = 10;

#[cfg(not(debug_assertions))]
const MAX_LOG_FOLDERS: usize = 3;

impl LogController {
    /// Creates a new [`LogController`]
    pub fn new<Game: GameMetadata>(options: &LoggingOptions<Game>) -> Result<Arc<Self>> {
        // Get the start time
        let start_time = Instant::now();

        // Create the message queue
        let (message_queue, message_queue_recv) = std::sync::mpsc::channel();

        // Create the outputs for the logger
        let outputs = create_outputs(options)?;

        let controller = Arc::new(LogController {
            minimum_severity: options.min_log_severity,
            frame: AtomicU64::new(0),
            start_time,
            message_queue,
            outputs: Mutex::new(Some((outputs, message_queue_recv))),
        });

        LogController::set(controller.clone());

        Ok(controller)
    }
}

fn create_outputs<Game: GameMetadata>(
    options: &LoggingOptions<Game>,
) -> Result<Vec<Box<dyn LogOutput>>> {
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
    if options.log_combined != FormatterKind::None || options.log_scoped != FormatterKind::None {
        std::fs::create_dir_all(&log_folder).map_err(|error| {
            new_error!("unable to create \"{}\" - {}", log_folder.display(), error,)
        })?;

        cleanup_log_folder(&base_folder).map_err(|error| {
            new_error!(
                "unable to cleanup \"{}\" - {}",
                base_folder.display(),
                error,
            )
        })?;
    }

    // Create outputs
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
