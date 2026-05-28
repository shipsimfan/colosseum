use crate::{GlobalSharedState, Result, logging::Logger, settings::SettingsCache};
use alexandria::gpu::{VulkanInstance, VulkanSurface};
use graphics_device::GraphicsDevice;

mod graphics_device;

/// Run the main game thread
pub(in crate::run) fn run<Game: crate::Game>(
    shared_state: &GlobalSharedState,
    instance: VulkanInstance,
    surface: VulkanSurface,
    settings: Game::SettingsCache,
    logger: Logger,
) -> Result<()> {
    let device = GraphicsDevice::new(
        settings.display_settings().adapter.as_deref(),
        &instance,
        &surface,
        &logger,
    )?;

    while shared_state.is_running() {
        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    Ok(())
}
