use std::fs::File;

mod drop;
mod new;
mod output;

/// Writes log messages to a combined file
pub(in crate::logging) struct CombinedFileOutput<Formatter: crate::logging::Formatter> {
    /// The file to write logs to
    file: File,

    /// The formatter to use for logs
    formatter: Formatter,
}
