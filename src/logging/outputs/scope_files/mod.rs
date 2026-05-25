use file::ScopeFile;
use std::path::PathBuf;

mod file;

mod new;
mod output;

/// Writes log messages to files based on scope
pub(in crate::logging) struct ScopeFilesOutput<Formatter: crate::logging::Formatter> {
    /// The currently open scope files
    files: Vec<ScopeFile<Formatter>>,

    /// The path to create log files in
    path: PathBuf,

    /// The base formatter to clone for each file
    base_formatter: Formatter,
}
