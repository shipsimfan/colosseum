use crate::logging::LogController;
use std::sync::Arc;

mod deref;
mod log;
mod new;

/// A logger for a specific scope
#[derive(Clone)]
pub struct Logger {
    /// A reference to the controller
    controller: Arc<LogController>,

    /// The scope to emit logs with
    scope: &'static str,
}
