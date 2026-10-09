use crate::logging::{LogSeverity, Logger};

impl Logger {
    /// Log a message
    pub fn log<S: Into<String>>(&self, severity: LogSeverity, message: S, module: &'static str) {
        self.controller
            .log(severity, message.into(), self.scope, module);
    }
}
