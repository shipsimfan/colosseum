use crate::{
    Result, Window,
    render::{GraphicsDevice, Swapchain},
};
use alexandria::{gpu::VulkanSurface, math::Vector2u};
use colosseum_ecs::World;
use std::time::Duration;

/// Copies the render-related data into render-specific structures
pub(in crate::render) struct PreRender;

/// The main render phase
pub(in crate::render) struct Render;

/// Performs any actions that need to take place after rendering has completed
pub(in crate::render) struct PostRender;

/// Execute all render-related phases
pub(crate) fn execute_render_systems(world: &mut World, surface: &mut VulkanSurface) -> Result<()> {
    if world.try_get_resource::<Swapchain>().is_some() {
        world.run::<PreRender>()?;
        world.run::<Render>()?;
        return world.run::<PostRender>();
    }

    // Check if we can create a swapchain
    let window = world.get_resource::<Window>();

    let window_size = window.size();
    if window_size != Vector2u::ZERO {
        drop(window);

        let mut graphics_device = world.get_resource_mut::<GraphicsDevice>();
        let swapchain = Swapchain::new(surface, window_size, &mut graphics_device)?;
        drop(graphics_device);

        world.insert_resource(swapchain);
    }

    // Sleep for a short duration to limit background FPS
    std::thread::sleep(Duration::from_millis(32)); // TODO: Change this to setting ("background FPS limit")
    Ok(())
}
