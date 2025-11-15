use crate::{
    Error, Result, RunningState,
    logging::{
        CombinedFileOutput, FormatterKind, HumanReadableFormatter, JsonFormatter, LogController,
        LogOutput, LoggingOptions, ScopeFilesOutput, StdoutOutput, log_thread,
    },
    util::expand_environment_string,
};
use std::{
    io::{IsTerminal, stdout},
    path::{Path, PathBuf},
    sync::{Arc, atomic::AtomicU64},
};
use time::{DateTime, NoTimeZone};
use win32::{
    LARGE_INTEGER, QueryPerformanceCounter, QueryPerformanceFrequency, try_get_last_error,
};

#[cfg(debug_assertions)]
const MAX_LOG_FOLDERS: usize = 10;

#[cfg(not(debug_assertions))]
const MAX_LOG_FOLDERS: usize = 3;

impl LogController {
    /// Creates a new [`LogController`]
    pub(crate) fn new<Game: crate::Game>(
        options: &LoggingOptions<Game>,
        running_state: Arc<RunningState>,
    ) -> Result<Arc<Self>> {
        // Get the performance counter frequency
        let mut performance_counter_frequency = LARGE_INTEGER::default();
        try_get_last_error!(QueryPerformanceFrequency(
            &mut performance_counter_frequency
        ))
        .map_err(|error| Error::new_inner("unable to get performance counter frequency", error))?;
        let performance_counter_frequency =
            unsafe { performance_counter_frequency.quad_part } as u64;

        // Get the start time
        let mut start_ticks = LARGE_INTEGER::default();
        try_get_last_error!(QueryPerformanceCounter(&mut start_ticks))
            .map_err(|error| Error::new_inner("unable to get performance counter", error))?;
        let start_ticks = unsafe { start_ticks.quad_part } as u64;

        // Build base path
        let base_folder = PathBuf::from(expand_environment_string(&options.log_folder.as_path())?);

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
                Error::new_inner(
                    format!("unable to create \"{}\"", log_folder.display()),
                    error,
                )
            })?;

            cleanup_log_folder(&base_folder).map_err(|error| {
                Error::new_inner(
                    format!("unable to cleanup \"{}\"", base_folder.display()),
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

        // Create the message queue
        let (message_queue, message_queue_recv) = std::sync::mpsc::channel();

        // Spawn the log thread
        let join_handle = Some(
            std::thread::Builder::new()
                .name("Logging".to_string())
                .spawn(move || log_thread(message_queue_recv, outputs, running_state))
                .map_err(|error| Error::new_inner("unable to spawn logger thread", error))?,
        );

        Ok(Arc::new(LogController {
            minimum_severity: options.min_log_severity,
            frame: AtomicU64::new(0),
            performance_counter_frequency,
            start_ticks,
            message_queue,
            join_handle,
        }))
    }
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
