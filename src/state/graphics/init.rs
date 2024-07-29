use super::{EventLogger, GraphicsState, Settings};
use crate::{info, logging::LogController, SettingsController, DEBUG};

impl GraphicsState {
    /// Creates a new [`GraphicsState`]
    pub(in crate::state) fn new(
        title: &str,
        log_controller: &LogController,
        settings_controller: &mut SettingsController,
    ) -> Result<Self, Box<dyn alexandria::Error>> {
        let logger = log_controller.logger("graphics");

        let settings: Settings = settings_controller.load()?;

        info!(logger, "Creating graphics instance");
        let instance = alexandria::Instance::new(if DEBUG {
            Some(EventLogger::new(log_controller.logger("vulkan")))
        } else {
            None
        })?;

        info!(logger, "Creating window");
        let window = alexandria::Window::new(title, settings.width(), settings.height())?;

        Ok(GraphicsState {
            logger,
            settings,
            instance,
            window,
        })
    }
}
