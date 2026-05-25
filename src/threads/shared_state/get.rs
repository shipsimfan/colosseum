use crate::{GlobalSharedState, logging::Logger};

impl GlobalSharedState {
    /// Get the logger for thread operations
    pub fn logger(&self) -> &Logger {
        &self.logger
    }
}
