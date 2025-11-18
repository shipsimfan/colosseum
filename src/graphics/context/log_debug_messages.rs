use crate::{Result, graphics::GraphicsContext};

impl GraphicsContext {
    /// Push all waiting debug messages into the log callbacks
    pub(in crate::graphics::context) fn log_debug_messages(&mut self) -> Result<()> {
        #[cfg(not(debug_assertions))]
        return Ok(());
        #[cfg(debug_assertions)]
        self.d3d11_info_queue.empty_queue()?;
        #[cfg(debug_assertions)]
        self.dxgi_info_queue.empty_queue()
    }
}
