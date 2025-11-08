use crate::logging::LogSeverity;
use argparse::FlagGroup;
use std::marker::PhantomData;

mod formatter_kind;
mod log_path;

pub(in crate::logging) use formatter_kind::FormatterKind;
pub(in crate::logging) use log_path::LogPath;

/// The default minimum log level to use for debug builds
#[cfg(debug_assertions)]
const DEFAULT_MIN_LOG_SEVERITY: LogSeverity = LogSeverity::Debug;

/// The default minimum log level to use for release builds
#[cfg(not(debug_assertions))]
const DEFAULT_MIN_LOG_SEVERITY: LogSeverity = LogSeverity::Info;

/// The options to control the logger
#[derive(FlagGroup)]
pub struct LoggingOptions<Game: crate::Game> {
    /// The folder to place log files into
    #[flag(value = "PATH", default = LogPath::Default(PhantomData))]
    pub(in crate::logging) log_folder: LogPath<Game>,

    /// The minimum severity of logs to display. Options are `error`, `warning`, `info`, and
    /// `debug`
    #[flag(value = "SEVERITY", default = DEFAULT_MIN_LOG_SEVERITY)]
    pub(in crate::logging) min_log_severity: LogSeverity,

    /// The format to use when logging to standard output. Options are `none`, `human`, `json`, and
    /// `json-pretty`
    #[flag(value = "FORMAT", default = FormatterKind::STDOUT_DEFAULT)]
    pub(in crate::logging) log_stdout: FormatterKind,

    /// The format to use when logging to a combined file. Options are `none`, `human`, `json`, and
    /// `json-pretty`
    #[flag(value = "FORMAT", default = FormatterKind::COMBINED_FILE_DEFAULT)]
    pub(in crate::logging) log_combined: FormatterKind,

    /// The format to use when logging to scoped files. Options are `none`, `human`, `json`, and
    /// `json-pretty`
    #[flag(value = "FORMAT", default = FormatterKind::SCOPED_FILES_DEFAULT)]
    pub(in crate::logging) log_scoped: FormatterKind,
}
