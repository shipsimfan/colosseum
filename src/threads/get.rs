use crate::{GlobalSharedState, ThreadManager};
use std::sync::Arc;

impl ThreadManager {
    /// Get a reference to the shared state
    pub fn shared_state(&self) -> &Arc<GlobalSharedState> {
        &self.shared_state
    }
}
