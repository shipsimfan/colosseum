use crate::logging::LogMessage;

mod combined_file;
mod scope_files;
mod stdout;

pub(in crate::logging) use combined_file::CombinedFileOutput;
pub(in crate::logging) use scope_files::ScopeFilesOutput;
pub(in crate::logging) use stdout::StdoutOutput;

/// An item which can manage log outputs
pub(in crate::logging) trait LogOutput: Send {
    /// Output a log `message`
    fn output(&mut self, message: &LogMessage);
}
