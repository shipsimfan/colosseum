use crate::{Window, run::Wsi};
use alexandria::EventQueue;

impl Wsi {
    /// Get a reference to the event queue the WSI thread consumes
    pub fn event_queue(&self) -> &EventQueue<()> {
        self.context.event_queue()
    }

    /// Get the shared window state
    pub fn window(&self) -> Window {
        Window::new(self.shared_window.clone())
    }
}
