use crate::RunningState;
use std::sync::atomic::Ordering;

impl RunningState {
    /// Is the engine currently running?
    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::Acquire)
    }
}
