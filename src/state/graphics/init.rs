use super::{EventLogger, GraphicsState};
use crate::{info, logging::LogController, DEBUG};

impl GraphicsState {
    /// Creates a new [`GraphicsState`]
    pub(in crate::state) fn new(
        title: &str,
        log_controller: &LogController,
    ) -> Result<Self, Box<dyn alexandria::Error>> {
        let logger = log_controller.logger("graphics");

        info!(logger, "Creating graphics instance");
        let instance = alexandria::Instance::new(if DEBUG {
            Some(EventLogger::new(log_controller.logger("vulkan")))
        } else {
            None
        })?;

        info!(logger, "Creating window");
        let window = alexandria::Window::new(title, 1280, 720)?;

        Ok(GraphicsState {
            logger,
            instance,
            window,
        })
    }
}
