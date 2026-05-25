use crate::logging::{LogController, LogSeverity};

impl LogController {
    /// Should a message with the given parameters be logged?
    pub fn should_log(&self, severity: LogSeverity) -> bool {
        severity <= self.minimum_severity
    }
}
