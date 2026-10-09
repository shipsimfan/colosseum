use crate::logging::LogMessage;
use std::io::Write;

mod human_readable;
mod json;

pub(in crate::logging) use human_readable::HumanReadableFormatter;
pub(in crate::logging) use json::JsonFormatter;

/// An item which can format log messages and write them
pub(in crate::logging) trait Formatter: Clone + Send {
    /// The extension that should be used for files
    const EXTENSION: &str;

    /// Format `message` and write it to `output`
    fn format(&mut self, message: &LogMessage, output: &mut dyn Write) -> std::io::Result<()>;

    /// Write a header to `output` if the format neads it
    #[allow(unused_variables)]
    fn start(&mut self, output: &mut dyn Write) -> std::io::Result<()> {
        Ok(())
    }

    /// Write a footer to `output` if the format needs it
    #[allow(unused_variables)]
    fn end(&mut self, output: &mut dyn Write) -> std::io::Result<()> {
        Ok(())
    }
}
