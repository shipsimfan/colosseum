use std::fmt::{Display, Formatter};

/// The severity of a log message
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    /// A message used for debugging
    Debug,

    /// A message used for normal information
    Info,

    /// A message about an warning state
    Warning,

    /// A message about an error that caused something to fail
    Error,

    /// A message about an error that has crashed the program
    Fatal,
}

impl Severity {
    /// Gets the severity as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Debug => "debug",
            Severity::Info => "info",
            Severity::Warning => "warn",
            Severity::Error => "error",
            Severity::Fatal => "fatal",
        }
    }

    /// Gets the color for the this severity as an ANSI code
    pub(super) fn color(&self) -> &'static str {
        match self {
            Severity::Debug => "37",
            Severity::Info => "36",
            Severity::Warning => "33",
            Severity::Error | Severity::Fatal => "31",
        }
    }
}

impl Display for Severity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
