use crate::{
    GlobalSharedState, Result, ThreadManager, Window,
    file_io::FileIo,
    info,
    logging::{LogController, Logger},
    render::RenderJob,
    settings::SettingsCache,
    update::UpdateJob,
};
use alexandria::{
    gpu::{VulkanInstance, VulkanSurface},
    math::Vector2u,
};
use std::{sync::Arc, time::Instant};

/// Run the main game thread
pub(in crate::run) fn run<Game: crate::Game>(
    shared_state: &GlobalSharedState,
    instance: VulkanInstance,
    mut surface: VulkanSurface,
    mut settings: Game::SettingsCache,
    options: Game::Options,
    window: Window,
    logger: Logger,
    log_controller: Arc<LogController>,
    file_io: FileIo,
    thread_manager: Arc<ThreadManager>,
) -> Result<()> {
    // Initialize the render and update jobs
    let mut window_size = window.size();
    let (mut render_job, transfer_queue) = RenderJob::new(
        settings.display_settings().adapter().as_deref(),
        &instance,
        &mut surface,
        window_size,
        &logger,
        &thread_manager,
    )?;

    let mut update_job = match UpdateJob::<Game>::new(
        &options,
        window_size,
        &logger,
        &mut settings,
        file_io,
        &window,
        transfer_queue,
        &mut render_job,
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
        // Get the window size for this frame atomically
        window_size = window.size();
        if window_size == Vector2u::ZERO {
            window.wait_for_restore()?;
            last_time = Instant::now(); // Reset the timer to avoid a large delta time after restoring the window
            continue;
        }

        // Reset per-frame data
        let render_data = render_job.render_data();
        render_data.reset();
        log_controller.frame();

        // Prepare the timing data for this frame
        let current_time = Instant::now();
        let delta_time = current_time - last_time;
        last_time = current_time;

        // Update and render the frame
        if !update_job.run(window_size, delta_time, render_data, &window)? {
            info!(logger, "Update job requested exit");
            break;
        }
        render_job = render_job.run(window_size)?;
    }

    Ok(())
}
