use std::io::Stdout;

mod drop;
mod new;
mod output;

/// Writes log messages to standard output
pub(in crate::logging) struct StdoutOutput<Formatter: crate::logging::Formatter> {
    /// A reference to the standard output
    stdout: Stdout,

    /// The formatter to use for logs
    formatter: Formatter,
}
