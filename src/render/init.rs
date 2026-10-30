use crate::{
    Result, SettingsCache, UserEvent,
    render::{
        Camera, FrameGraph, GraphicsDevice, RenderGpuTransferQueue, RenderIndices, Swapchain,
        create_fixed_render_data_resources, create_frame_graph_resources,
        create_per_frame_render_data, create_public_data_resources,
        create_public_data_scene_resources,
        phases::{self, FrameGraphSetup, PostRender, PreRender},
    },
};
use alexandria::gpu::{VulkanInstance, VulkanSurface};
use colosseum_core::ThreadManager;
use colosseum_ecs::World;
use std::sync::Arc;

/// Initialize all the resources required for rendering
pub(crate) fn init_render_resources<Game: crate::Game>(
    instance: &VulkanInstance,
    surface: &VulkanSurface,
    thread_manager: &ThreadManager<UserEvent>,
    world: &mut World,
) -> Result<(Arc<GraphicsDevice>, RenderIndices<Game>)> {
    // Insert the settings
    let settings = world.get_resource::<Game::SettingsCache>();

    // Create the graphics device
    let (device, mut render_queue, gpu_transfer_queue, mut render_gpu_transfer_queue) =
        GraphicsDevice::new(
            settings.display_settings().adapter().as_deref(),
            &instance,
            surface,
            &thread_manager,
        )?;

    drop(settings);

    // Create the per-frame data resources
    create_per_frame_render_data(world, &device)?;

    // Create and insert the global graphics resources
    create_fixed_render_data_resources(world, &device)?;
    create_frame_graph_resources::<Game>(world, &device)?;
    create_public_data_resources(
        world,
        &device,
        gpu_transfer_queue,
        render_gpu_transfer_queue
            .as_mut()
            .map(|transfer_queue| (transfer_queue, &mut *render_queue)),
    )?;

    // Insert render resources
    world.insert_permanent_resource(render_queue);
    if let Some(render_gpu_transfer_queue) = render_gpu_transfer_queue {
        world.insert_permanent_resource(render_gpu_transfer_queue);
    }

    Ok((device, RenderIndices::new(world)))
}

/// Initialize the all the required render systems
pub(crate) fn init_render_systems<Game: crate::Game>(
    world: &mut World,
    indices: &RenderIndices<Game>,
) {
    create_public_data_scene_resources(world);
    indices.frame_graph_mut(world).reset();

    init_pre_render(world);
    init_frame_graph_setup::<Game>(world, indices);
    world.register_phase(phases::FrameGraph);
    init_post_render(world);
}

fn init_pre_render(world: &mut World) {
    if world.try_get_resource::<RenderGpuTransferQueue>().is_some() {
        world.register_system(PreRender, RenderGpuTransferQueue::system);
    }

    world.register_system(PreRender, Camera::system);
}

fn init_frame_graph_setup<Game: crate::Game>(world: &mut World, indices: &RenderIndices<Game>) {
    world.register_system(FrameGraphSetup, indices.gpu_data_buffers(world).system());
    world.register_system(FrameGraphSetup, Swapchain::acquire_next_image);
    FrameGraph::<Game>::register_prepare(world);
}

fn init_post_render(world: &mut World) {
    world.register_system(PostRender, Swapchain::present);
}
