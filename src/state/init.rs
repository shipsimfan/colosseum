use super::Colosseum;
use crate::{logging::LogController, state::graphics::GraphicsState, SettingsController};
use std::path::Path;

impl Colosseum {
    pub(crate) fn new(
        title: &str,
        log_directory: Option<&Path>,
        settings_directory: Option<&Path>,
    ) -> Result<Self, Box<dyn alexandria::Error>> {
        let settings = SettingsController::new(settings_directory);

        let log_controller = LogController::new(log_directory);

        let graphics = GraphicsState::new(title, &log_controller)?;

        Ok(Colosseum {
            graphics,
            log_controller,
            settings,
        })
    }
}
