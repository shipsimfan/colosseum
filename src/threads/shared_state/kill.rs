use crate::{GlobalSharedState, debug};
use alexandria::EventKind;
use std::sync::atomic::Ordering;

impl GlobalSharedState {
    /// Signal all threads to stop running
    pub fn kill(&self) {
        debug!(&self.logger, "killing all threads");

        self.is_running.store(false, Ordering::Release);

        let event_queue = self.event_queue.lock().unwrap();
        if let Some(event_queue) = event_queue.as_ref() {
            event_queue.push(EventKind::Quit).ok();
        }
    }
}
