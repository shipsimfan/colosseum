use crate::{
    Result, SettingsCache, UserEvent,
    render::{
        FrameGraph, GraphicsDevice, Swapchain, create_fixed_render_data_resources,
        create_per_frame_render_data, create_render_data_resources,
        phases::{FrameGraphSetup, PostRender},
    },
};
use alexandria::gpu::{VulkanInstance, VulkanSurface};
use colosseum_core::ThreadManager;
use colosseum_ecs::World;

/// Initialize all the resources required for rendering
pub(crate) fn init_render_resources<Game: crate::Game>(
    instance: &VulkanInstance,
    surface: &VulkanSurface,
    thread_manager: &ThreadManager<UserEvent>,
    world: &mut World,
) -> Result<()> {
    // Insert the settings
    let settings = world.get_resource::<Game::SettingsCache>();

    // Create the graphics device
    let (graphics_device, transfer_queue) = GraphicsDevice::new(
        settings.display_settings().adapter().as_deref(),
        &instance,
        surface,
        &thread_manager,
    )?;

    drop(settings);

    // Create the per-frame data resources
    create_per_frame_render_data(world, &graphics_device)?;

    // Create and insert the global graphics resources
    create_fixed_render_data_resources(world, &graphics_device)?;
    create_render_data_resources(world);

    world.insert_resource(graphics_device);
    world.insert_resource(transfer_queue);

    // Initialize the render systems
    // TODO: Move this to scene transition
    init_render_systems(world);

    Ok(())
}

/// Initialize the all the required render systems
pub(crate) fn init_render_systems(world: &mut World) {
    init_pre_render();
    init_frame_graph_setup(world);
    init_frame_graph(world);
    init_post_render(world);
}

fn init_pre_render() {}

fn init_frame_graph_setup(world: &mut World) {
    world.register_system(FrameGraphSetup, Swapchain::acquire_next_image);
}

fn init_frame_graph(world: &mut World) {
    FrameGraph::new().compile(world);
}

fn init_post_render(world: &mut World) {
    world.register_system(PostRender, Swapchain::present);
}
