use crate::logging::LogSeverity;

impl std::fmt::Display for LogSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogSeverity::Error => "error",
            LogSeverity::Warning => "warning",
            LogSeverity::Info => "info",
            LogSeverity::Debug => "debug",
        }
        .fmt(f)
    }
}
