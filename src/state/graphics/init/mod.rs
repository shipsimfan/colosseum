use super::{EventLogger, GraphicsState, Settings};
use crate::{
    info, logging::LogController, state::graphics::select_physical_device::select_physical_device,
    SettingsController, DEBUG,
};

mod error;

pub use error::GraphicsInitError;

impl GraphicsState {
    /// Creates a new [`GraphicsState`]
    pub(in crate::state) fn new(
        title: &str,
        log_controller: &LogController,
        settings_controller: &mut SettingsController,
    ) -> Result<Self, GraphicsInitError> {
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

        let physical_devices = instance.physical_devices(&window)?;
        for physical_device in &physical_devices {
            info!(
                logger,
                "Available physical device: {}",
                physical_device.name()
            );
        }
        let selected_physical_device = select_physical_device(settings.device(), &physical_devices)
            .ok_or(GraphicsInitError::NoSupportedPhysicalDevices)?;
        info!(
            logger,
            "Selected physical device: {}",
            selected_physical_device.name()
        );

        let device = alexandria::Device::new(selected_physical_device)?;

        Ok(GraphicsState {
            device,
            window,
            instance,
            settings,
            logger,
        })
    }
}
