use crate::logging::{LogController, Logger};
use std::sync::Arc;

impl Logger {
    /// Create a new [`Logger`]
    pub(in crate::logging) fn new(controller: Arc<LogController>, scope: &'static str) -> Self {
        Logger { controller, scope }
    }
}
