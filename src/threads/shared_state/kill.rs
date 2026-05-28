use crate::{GlobalSharedState, debug};
use alexandria::EventKind;
use std::sync::atomic::Ordering;

impl GlobalSharedState {
    /// Signal all threads to stop running
    pub fn kill(&self) {
        let old_value = self.is_running.swap(false, Ordering::Release);
        if old_value {
            debug!(&self.logger, "Killing all threads");
        }

        let event_queue = self.event_queue.lock().unwrap();
        if let Some(event_queue) = event_queue.as_ref() {
            event_queue.push(EventKind::Quit).ok();
        }
    }
}
