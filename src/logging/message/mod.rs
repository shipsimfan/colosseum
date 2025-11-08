use data_format::Serialize;

mod get;
mod new;
mod severity;

pub use severity::LogSeverity;

/// A single compiled log message
#[derive(Debug, Serialize)]
pub(in crate::logging) struct LogMessage {
    /// The severity of the message
    severity: LogSeverity,

    /// The contents of the message
    message: String,

    /// The frame the log message was generated on
    frame: u64,

    /// The number of milliseconds after the program launched that this message was generated at
    milliseconds: u64,

    /// The scope that emitted the message
    scope: &'static str,

    /// The code module that emitted the message
    module: &'static str,
}
