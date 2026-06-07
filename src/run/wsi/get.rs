use crate::{
    InputEvent, UserEvent, Window,
    run::{Wsi, wsi::SharedWindow},
};
use alexandria::EventQueue;
use std::sync::{Arc, mpsc::Receiver};

impl Wsi {
    /// Get a reference to the event queue the WSI thread consumes
    pub fn event_queue(&self) -> &EventQueue<UserEvent> {
        self.context.event_queue()
    }

    /// Get the shared window state
    pub fn window(&self, inputs: Receiver<InputEvent>) -> Window {
        Window::new(
            self.shared_window.clone(),
            inputs,
            self.context.event_queue().clone(),
        )
    }

    /// Signal the restored notify to kill any threads waiting for the window to be restored
    pub fn shared_window(&self) -> &Arc<SharedWindow> {
        &self.shared_window
    }
}
