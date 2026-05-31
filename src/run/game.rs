use crate::{
    GlobalSharedState, Result, Window,
    logging::{LogController, Logger},
    render::RenderJob,
    settings::SettingsCache,
};
use alexandria::gpu::{VulkanInstance, VulkanSurface};
use std::sync::Arc;

/// Run the main game thread
pub(in crate::run) fn run<Game: crate::Game>(
    shared_state: &GlobalSharedState,
    instance: VulkanInstance,
    surface: VulkanSurface,
    settings: Game::SettingsCache,
    window: Window,
    logger: Logger,
    log_controller: Arc<LogController>,
) -> Result<()> {
    let mut render_job = RenderJob::new(
        settings.display_settings().adapter.as_deref(),
        &instance,
        &surface,
        window.size(),
        &logger,
    )?;

    while shared_state.is_running() {
        render_job = render_job.run(window.size())?;

        log_controller.frame();
    }

    Ok(())
}
