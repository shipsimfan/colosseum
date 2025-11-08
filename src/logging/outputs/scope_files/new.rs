use crate::logging::ScopeFilesOutput;
use std::path::PathBuf;

impl<Formatter: crate::logging::Formatter> ScopeFilesOutput<Formatter> {
    /// Create a new [`ScopeFilesOutput`]
    pub fn new(path: PathBuf, formatter: Formatter) -> Self {
        ScopeFilesOutput {
            files: Vec::new(),
            path,
            base_formatter: formatter,
        }
    }
}
