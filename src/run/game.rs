use crate::{
    GlobalSharedState, Result, logging::Logger, render::RenderJob, settings::SettingsCache,
};
use alexandria::{
    gpu::{VulkanInstance, VulkanSurface},
    math::Vector2u,
};

/// Run the main game thread
pub(in crate::run) fn run<Game: crate::Game>(
    shared_state: &GlobalSharedState,
    instance: VulkanInstance,
    surface: VulkanSurface,
    settings: Game::SettingsCache,
    window_size: Vector2u,
    logger: Logger,
) -> Result<()> {
    let mut render_job = RenderJob::new(
        settings.display_settings().adapter.as_deref(),
        &instance,
        &surface,
        window_size,
        &logger,
    )?;

    while shared_state.is_running() {
        render_job = render_job.run(window_size)?;
    }

    Ok(())
}
