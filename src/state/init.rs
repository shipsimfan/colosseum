use super::Colosseum;
use crate::{logging::LogController, state::graphics::GraphicsState, SettingsController};
use std::path::Path;

impl Colosseum {
    pub(crate) fn new(
        title: &str,
        log_directory: Option<&Path>,
        settings_directory: Option<&Path>,
    ) -> Result<Self, Box<dyn alexandria::Error>> {
        let log_controller = LogController::new(log_directory);

        let mut settings = SettingsController::new(settings_directory, &log_controller);

        let graphics = GraphicsState::new(title, &log_controller, &mut settings)?;

        Ok(Colosseum {
            graphics,
            log_controller,
            settings,
        })
    }
}
