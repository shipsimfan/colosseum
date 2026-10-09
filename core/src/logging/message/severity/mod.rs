use data_format::Serialize;

mod color;
mod display;
mod flag;

/// The severity of a log message
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub enum LogSeverity {
    /// The log message describes an error
    Error,

    /// The log message describes a warning
    Warning,

    /// The log message contains some information
    Info,

    /// The log message is meant for debugging
    Debug,
}
