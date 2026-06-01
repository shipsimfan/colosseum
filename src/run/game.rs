use crate::{
    GlobalSharedState, Result, Window, info,
    logging::{LogController, Logger},
    render::{RenderData, RenderJob},
    settings::SettingsCache,
    update::UpdateJob,
};
use alexandria::gpu::{VulkanInstance, VulkanSurface};
use std::{sync::Arc, time::Instant};

/// Run the main game thread
pub(in crate::run) fn run<Game: crate::Game>(
    shared_state: &GlobalSharedState,
    instance: VulkanInstance,
    surface: VulkanSurface,
    mut settings: Game::SettingsCache,
    options: Game::Options,
    window: Window,
    logger: Logger,
    log_controller: Arc<LogController>,
) -> Result<()> {
    // Initialize the render and update jobs
    let mut window_size = window.size();
    let mut render_job = RenderJob::new(
        settings.display_settings().adapter.as_deref(),
        &instance,
        &surface,
        window_size,
        &logger,
    )?;
    let mut render_data = RenderData::new();

    let mut update_job = match UpdateJob::<Game>::new(
        &options,
        window_size,
        &logger,
        &mut settings,
        &mut render_data,
    )? {
        Some(job) => job,
        None => {
            info!(logger, "Initial scene requested exit");
            return Ok(());
        }
    };

    // Run the main loop, breaking if either the update job or the thread manager indicates that
    // the game should exit
    let mut last_time = Instant::now();
    while shared_state.is_running() {
        render_data.reset();
        log_controller.frame();

        // Prepare the timing data for this frame
        let current_time = Instant::now();
        let delta_time = current_time - last_time;
        last_time = current_time;

        // Get the window size for this frame atomically
        window_size = window.size();

        // Update and render the frame
        if !update_job.run(window_size, delta_time, &mut render_data, &window)? {
            info!(logger, "Update job requested exit");
            break;
        }
        render_job = render_job.run(window_size, &render_data)?;
    }

    Ok(())
}
