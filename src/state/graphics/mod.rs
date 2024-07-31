use crate::{info, logging::Logger};
use alexandria::{Device, Instance, Window};
use event_logger::EventLogger;
use settings::Settings;

mod event_logger;
mod init;
mod select_physical_device;
mod settings;

/// The current graphics state
pub(super) struct GraphicsState {
    /// The device used to render
    device: Device,

    /// The window the game is displayed in
    window: Window,

    /// The graphics instance
    instance: Instance,

    /// The graphics settings
    settings: Settings,

    /// The logger for graphics information
    logger: Logger,
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
