use super::Colosseum;
use crate::{event_logger::EventLogger, info, logging::LogController};
use std::path::Path;

#[cfg(debug_assertions)]
const DEBUG: bool = true;

#[cfg(not(debug_assertions))]
const DEBUG: bool = false;

impl Colosseum {
    pub(crate) fn new(
        title: &str,
        log_directory: Option<&Path>,
    ) -> Result<Self, Box<dyn alexandria::Error>> {
        let log_controller = LogController::new(log_directory);
        let graphics_logger = log_controller.logger("graphics");

        info!(graphics_logger, "Creating graphics instance");
        let instance = alexandria::Instance::new(if DEBUG {
            Some(EventLogger::new(log_controller.logger("vulkan")))
        } else {
            None
        })?;

        info!(graphics_logger, "Creating window");
        let window = alexandria::Window::new(title, 1280, 720)?;

        Ok(Colosseum {
            instance,
            window,
            graphics_logger,
            log_controller,
        })
    }
}
