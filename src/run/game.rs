use crate::{
    Result, UserEvent, Window,
    file_io::FileIo,
    render::{execute_render_systems, init_render_resources, stop_render},
    update::{Scene, execute_update_systems},
};
use alexandria::gpu::{VulkanInstance, VulkanSurface};
use colosseum_core::{GlobalSharedState, ThreadManager, logger};
use colosseum_ecs::World;
use std::sync::Arc;

/// Run the main game thread
pub(in crate::run) fn run<Game: crate::Game, InitialScene: Scene>(
    settings: Game::SettingsCache,
    options: Game::Options,
    initial_scene: InitialScene,

    window: Window,
    instance: VulkanInstance,
    mut surface: VulkanSurface,

    shared_state: &GlobalSharedState<UserEvent>,
    file_io: FileIo,
    thread_manager: Arc<ThreadManager<UserEvent>>,
) -> Result<()> {
    let logger = logger!("game");

    let mut world = World::new();
    world.insert_resource(settings);
    world.insert_resource(options);
    world.insert_resource(window);

    init_render_resources::<Game>(&instance, &surface, &thread_manager, &mut world)?;

    initial_scene.start(&mut world)?;

    while shared_state.is_running() {
        logger.frame();

        execute_update_systems(&mut world)?;
        execute_render_systems(&mut world, &mut surface)?;
    }

    stop_render(&mut world)?;

    Ok(())

    /*
    // Initialize the render and update jobs
    let mut update_job = UpdateJob::<Game>::new(
        options,
        &logger,
        &mut settings,
        file_io,
        transfer_queue,
        &mut render_job,
    )?;

    // Run the main loop, breaking if either the update job or the thread manager indicates that
    // the game should exit
    let mut last_time = Instant::now();
    while shared_state.is_running() {
        // Get the window size for this frame atomically
        window_size = window.size();
        if window_size == Vector2u::ZERO {
            debug!(logger, "Window size is zero, waiting for restore");
            window.wait_for_restore()?;
            last_time = Instant::now(); // Reset the timer to avoid a large delta time after restoring the window
            continue;
        }

        // Reset per-frame data
        let render_data = render_job.render_data();
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
        render_job = render_job.run(window_size, &window)?;
    }

    Ok(())
    */
}
