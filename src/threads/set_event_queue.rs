use crate::ThreadManager;
use alexandria::EventQueue;

impl ThreadManager {
    /// Set the event queue for communicating to the WSI
    pub fn set_event_queue(&self, event_queue: EventQueue<()>) {
        self.shared_state.set_event_queue(event_queue);
    }
}
