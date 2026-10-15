use crate::{
    Result, Window,
    render::{DrawFence, FrameIndex, GraphicsDevice, Swapchain},
};
use alexandria::{gpu::VulkanSurface, math::Vector2u};
use colosseum_ecs::{PhaseName, World};
use std::time::Duration;

/// Copies the render-related data into device-visible buffers
#[derive(PartialEq, Eq)]
pub(in crate::render) struct PreRender;

impl PhaseName for PreRender {}

/// Performs any needed setup for the frame graph, such as compiling the frame graph, acquiring the
/// swapchain image, or copying data into device-local buffers
#[derive(PartialEq, Eq)]
pub(in crate::render) struct FrameGraphSetup;

impl PhaseName for FrameGraphSetup {}

/// The main frame graph execution phase
#[derive(PartialEq, Eq)]
pub(in crate::render) struct FrameGraph;

impl PhaseName for FrameGraph {}

/// Submits all the command buffers and queues a present
#[derive(PartialEq, Eq)]
pub(in crate::render) struct PostRender;

impl PhaseName for PostRender {}

/// Execute all render-related phases
pub(crate) fn execute_render_systems(world: &mut World, surface: &mut VulkanSurface) -> Result<()> {
    if world.try_get_resource::<Swapchain>().is_some() {
        let swapchain_ok = render_frame(world)?;
        world.get_resource_mut::<FrameIndex>().increment();
        if swapchain_ok {
            return Ok(());
        }

        world.remove_resource::<Swapchain>();
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

/// Runs the needed render phases and returns if the swapchain is still valid
fn render_frame(world: &mut World) -> Result<bool> {
    let frame_index = world.get_resource::<FrameIndex>().clone();

    // TODO: Wait for copy fence
    world.run(PreRender)?;

    world.get_resource_mut::<DrawFence>().wait(&frame_index)?;
    world.run(FrameGraphSetup)?;
    if !world.get_resource::<Swapchain>().is_valid() {
        return Ok(false);
    }

    world.run(FrameGraph)?;
    world.run(PostRender)?;

    Ok(!world.get_resource::<Swapchain>().is_suboptimal())
}
