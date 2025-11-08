use std::fs::File;

mod drop;
mod get;
mod open;
mod output;

/// A file opened for a specific scope
pub(in crate::logging::outputs::scope_files) struct ScopeFile<Formatter: crate::logging::Formatter>
{
    /// The scope this file is for
    scope: &'static str,

    /// The file to write to
    file: File,

    /// The formatter for this file
    formatter: Formatter,
}
