use crate::run::Wsi;
use alexandria::{EventQueue, math::Vector2u};

impl Wsi {
    /// Get a reference to the event queue the WSI thread consumes
    pub fn event_queue(&self) -> &EventQueue<()> {
        self.context.event_queue()
    }

    /// Get the current size of the window that the WSI is managing
    pub fn window_size(&self) -> Vector2u {
        let size = self.window.size();
        Vector2u::new(size.x as _, size.y as _)
    }
}
