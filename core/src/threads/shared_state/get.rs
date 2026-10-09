use crate::{GlobalSharedState, Logger};
use std::sync::atomic::Ordering;

impl<UserEvent: 'static + Send> GlobalSharedState<UserEvent> {
    /// Get the logger for thread operations
    pub fn logger(&self) -> &Logger {
        &self.logger
    }

    /// Is the program still running?
    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }
}
