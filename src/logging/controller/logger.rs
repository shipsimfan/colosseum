use crate::logging::{LogController, Logger};
use std::sync::Arc;

impl LogController {
    /// Creates a new [`Logger`] with `scope`
    pub fn logger(self: &Arc<Self>, scope: &'static str) -> Logger {
        Logger::new(self.clone(), scope)
    }
}
