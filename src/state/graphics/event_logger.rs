use crate::logging::Logger;
use alexandria::EventCallback;
use std::{borrow::Cow, fmt::Display};

/// The logger used for Vulkan events
pub(super) struct EventLogger(Logger);

/// Display the list contained only if it contains elements
struct ObjectDisplay<'a>(&'a [Cow<'a, str>]);

impl EventLogger {
    /// Creates a new [`EventLogger`] to `logger`
    pub(super) fn new(logger: Logger) -> Box<dyn EventCallback> {
        Box::new(EventLogger(logger))
    }
}

impl EventCallback for EventLogger {
    fn callback(&self, severity: alexandria::Severity, message: &str, objects: Vec<Cow<str>>) {
        self.0.log(
            severity,
            &format_args!("{}{}", message, ObjectDisplay(&objects)),
        )
    }
}

impl<'a> Display for ObjectDisplay<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.len() == 0 {
            return Ok(());
        }

        write!(f, " ({:?})", self.0)
    }
}
