use crate::logging::{LogMessage, LogSeverity};

impl LogMessage {
    /// Get the severity of the this message
    pub fn severity(&self) -> LogSeverity {
        self.severity
    }

    /// Get the contained message
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Get the frame this message was emitted on
    pub fn frame(&self) -> u64 {
        self.frame
    }

    /// Get the scope that emitted this message
    pub fn scope(&self) -> &'static str {
        self.scope
    }

    /// Get the module that emitted this message
    pub fn module(&self) -> &'static str {
        self.module
    }
}
