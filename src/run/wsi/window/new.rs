use crate::{InputEvent, UserEvent, Window, run::wsi::SharedWindow};
use alexandria::EventQueue;
use std::sync::{Arc, mpsc::Receiver};

impl Window {
    /// Create a new [`Window`] access structure
    pub(in crate::run::wsi) fn new(
        shared: Arc<SharedWindow>,
        inputs: Receiver<InputEvent>,
        event_queue: EventQueue<UserEvent>,
    ) -> Window {
        Window {
            shared,
            inputs,
            event_queue,
        }
    }
}
