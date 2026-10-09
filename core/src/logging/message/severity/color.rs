use crate::logging::LogSeverity;

impl LogSeverity {
    /// Get the ANSI escape sequence for this log severity
    pub(in crate::logging) fn color(&self) -> &'static str {
        match self {
            LogSeverity::Error => "\x1B[31m",
            LogSeverity::Warning => "\x1B[33m",
            LogSeverity::Info => "\x1B[36m",
            LogSeverity::Debug => "\x1B[32m",
        }
    }
}
