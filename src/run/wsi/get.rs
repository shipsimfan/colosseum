use crate::{InputEvent, Window, run::Wsi};
use alexandria::EventQueue;
use std::sync::mpsc::Receiver;

impl Wsi {
    /// Get a reference to the event queue the WSI thread consumes
    pub fn event_queue(&self) -> &EventQueue<()> {
        self.context.event_queue()
    }

    /// Get the shared window state
    pub fn window(&self, inputs: Receiver<InputEvent>) -> Window {
        Window::new(self.shared_window.clone(), inputs)
    }
}
