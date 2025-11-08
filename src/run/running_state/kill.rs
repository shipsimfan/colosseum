use crate::RunningState;
use std::sync::atomic::Ordering;

impl RunningState {
    /// Signal that the engine should stop running
    pub fn kill(&self) {
        self.is_running.store(false, Ordering::Release);
    }
}
