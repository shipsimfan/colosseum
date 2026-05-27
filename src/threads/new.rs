use crate::{GlobalSharedState, ThreadManager, logging::LogController};
use std::sync::{Arc, Mutex};

impl ThreadManager {
    /// Create a new [`ThreadManager`]
    pub fn new(log_controller: &Arc<LogController>) -> Arc<ThreadManager> {
        Arc::new(ThreadManager {
            shared_state: Arc::new(GlobalSharedState::new(log_controller)),
            threads: Mutex::new(Vec::new()),
        })
    }
}
