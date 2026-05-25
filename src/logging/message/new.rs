use crate::logging::{LogMessage, LogSeverity};

impl LogMessage {
    /// Create a new [`LogMessage`]
    pub fn new(
        severity: LogSeverity,
        message: String,
        frame: u64,
        milliseconds: u64,
        scope: &'static str,
        module: &'static str,
    ) -> Self {
        LogMessage {
            severity,
            message,
            frame,
            milliseconds,
            scope,
            module,
        }
    }
}
