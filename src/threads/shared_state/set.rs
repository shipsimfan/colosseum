use crate::GlobalSharedState;
use std::sync::atomic::Ordering;

impl GlobalSharedState {
    /// Signal all threads to stop running
    pub fn kill(&self) {
        self.is_running.store(false, Ordering::Release);
    }
}
