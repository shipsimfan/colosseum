use crate::{
    Result, UserEvent, Window,
    file_io::FileIo,
    render::{execute_render_systems, init_render_resources, init_render_systems, stop_render},
    update::{
        NextScene, Scene, SceneTransition, execute_update_systems, init_update_resources,
        init_update_systems,
    },
};
use alexandria::gpu::{VulkanInstance, VulkanSurface};
use colosseum_core::{GlobalSharedState, ThreadManager, logger};
use colosseum_ecs::World;
use std::{sync::Arc, time::Instant};

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

    // Create world and insert basic resources
    let mut world = World::new();
    world.insert_permanent_resource(settings);
    world.insert_permanent_resource(options);
    world.insert_permanent_resource(window);
    world.insert_permanent_resource(file_io);

    // Create the graphics device and initialize render resources
    let (graphics_device, swapchain_index, frame_index_index, frame_graph_index) =
        init_render_resources::<Game>(&instance, &surface, &thread_manager, &mut world)?;

    // Initialize update resources
    init_update_resources::<Game, InitialScene>(&mut world, initial_scene);

    // Get the index of the resources we use in the main loop
    let settings_index = world.get_or_reserve_permanent_resource_index::<Game::SettingsCache>();
    let window_index = world.get_or_reserve_permanent_resource_index::<Window>();
    let next_scene_index = world.get_or_reserve_permanent_resource_index::<NextScene>();

    // Begin the initial scene
    let mut last_render_time = Instant::now();

    // Main game loop
    while shared_state.is_running() {
        // Check if the window should close
        let window = unsafe { world.get_resource_at_index::<Window>(window_index) };
        if window.should_close() {
            break;
        }
        drop(window);

        // Increment the frame counter of the logger
        logger.frame();

        // Transition to the next scene if it exists
        let mut next_scene =
            unsafe { world.get_resource_at_index_mut::<NextScene>(next_scene_index) };
        if let Some(next_scene) = next_scene.take() {
            world.run(SceneTransition)?;
            unsafe { world.clear() };

            init_render_systems::<Game>(&mut world, frame_graph_index);
            init_update_systems::<Game>(&mut world);

            next_scene.start(&mut world)?;
        }

        // Execute the frame
        execute_update_systems(&mut world)?;
        execute_render_systems::<Game>(
            &mut world,
            &mut surface,
            &mut last_render_time,
            window_index,
            settings_index,
            swapchain_index,
            frame_index_index,
            &graphics_device,
        )?;
    }

    // Wait for idle on the graphics device before stopping render
    stop_render(&graphics_device)?;

    Ok(())
}
