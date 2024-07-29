use crate::{info, logging::Logger};
use alexandria::{Instance, Window};
use event_logger::EventLogger;

mod event_logger;
mod init;

/// The current graphics state
pub(super) struct GraphicsState {
    /// The logger for graphics information
    logger: Logger,

    /// The graphics instance
    instance: Instance,

    /// The window the game is displayed in
    window: Box<Window>,
}

impl GraphicsState {
    /// Gets the window the game is displayed in
    pub(super) fn window(&mut self) -> &mut Window {
        &mut self.window
    }
}

impl Drop for GraphicsState {
    fn drop(&mut self) {
        info!(self.logger, "Shutting down");
    }
}
