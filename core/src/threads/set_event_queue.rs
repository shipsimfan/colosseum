use crate::ThreadManager;
use alexandria::EventQueue;

impl<UserEvent: 'static + Send> ThreadManager<UserEvent> {
    /// Set the event queue for communicating to the WSI
    pub fn set_event_queue(&self, event_queue: EventQueue<UserEvent>) {
        self.shared_state.set_event_queue(event_queue);
    }
}
