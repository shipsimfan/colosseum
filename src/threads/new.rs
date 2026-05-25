use crate::{GlobalSharedState, ThreadManager, logging::LogController};
use std::sync::Arc;

impl ThreadManager {
    /// Create a new [`ThreadManager`]
    pub fn new(log_controller: &Arc<LogController>) -> ThreadManager {
        ThreadManager {
            shared_state: Arc::new(GlobalSharedState::new(log_controller)),
            threads: Vec::new(),
        }
    }
}
