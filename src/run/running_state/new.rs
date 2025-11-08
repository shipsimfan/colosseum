use crate::RunningState;
use std::sync::{Arc, atomic::AtomicBool};

impl RunningState {
    /// Create a new [`RunningState`]
    pub(in crate::run) fn new() -> Arc<Self> {
        Arc::new(RunningState {
            is_running: AtomicBool::new(true),
        })
    }
}
