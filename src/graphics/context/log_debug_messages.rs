use crate::{Result, graphics::GraphicsContext};

impl GraphicsContext {
    /// Push all waiting debug messages into the log callbacks
    pub(in crate::graphics::context) fn log_debug_messages(&mut self) -> Result<()> {
        #[cfg(not(debug_assertions))]
        return Ok(());
        #[cfg(debug_assertions)]
        self.info_queue.empty_queue()
    }
}
