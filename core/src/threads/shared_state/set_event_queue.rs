use crate::{GlobalSharedState, UserEvent};
use alexandria::EventQueue;

impl GlobalSharedState {
    /// Set the event queue for communicating to the WSI
    pub(in crate::threads) fn set_event_queue(&self, event_queue: EventQueue<UserEvent>) {
        *self.event_queue.lock().unwrap() = Some(event_queue);
    }
}
