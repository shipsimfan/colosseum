use crate::{
    Result, SettingsCache, UserEvent,
    render::{AcquireImageSemaphore, DrawFence, FrameIndex, GraphicsDevice, Swapchain},
};
use alexandria::gpu::{VulkanInstance, VulkanSurface};
use colosseum_core::ThreadManager;
use colosseum_ecs::World;

/// Initialize all the objects required for rendering
pub(crate) fn init_render<Game: crate::Game>(
    instance: &VulkanInstance,
    surface: &VulkanSurface,
    thread_manager: &ThreadManager<UserEvent>,
    world: &mut World,
) -> Result<()> {
    // Insert the necessary render-related resources into the ECS world
    let settings = world.get_resource::<Game::SettingsCache>();

    let (graphics_device, transfer_queue) = GraphicsDevice::new(
        settings.display_settings().adapter().as_deref(),
        &instance,
        surface,
        &thread_manager,
    )?;

    drop(settings);

    world.insert_resource(DrawFence::new(&graphics_device)?);
    world.insert_resource(AcquireImageSemaphore::new(&graphics_device)?);

    world.insert_resource(graphics_device);
    world.insert_resource(transfer_queue);

    Swapchain::register_render_frame_system(world);
    FrameIndex::new(world);

    Ok(())
}
