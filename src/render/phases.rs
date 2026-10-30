use crate::{
    Result, SettingsCache,
    render::{GraphicsDevice, RenderIndices, Swapchain},
};
use alexandria::{gpu::VulkanSurface, math::Vector2u};
use colosseum_ecs::{PhaseName, World};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

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
pub(crate) fn execute_render_systems<Game: crate::Game>(
    world: &mut World,
    surface: &mut VulkanSurface,
    last_render_time: &mut Instant,

    device: &Arc<GraphicsDevice>,
    indices: &RenderIndices<Game>,
) -> Result<()> {
    let settings = indices.settings(world);
    let display_settings = settings.display_settings();
    let max_frame_rate = display_settings.maximum_frame_rate();
    let min_frame_rate = display_settings.minimized_frame_rate();

    if indices.try_swapchain(world).is_some() {
        let swapchain_ok = render_frame(world, indices)?;

        indices.frame_index_mut(world).increment();

        if swapchain_ok {
            if let Some(max_frame_rate) = max_frame_rate {
                // Limit frame rate based on max_frame_rate
                let frame_duration = Duration::from_secs_f32(1.0 / max_frame_rate);
                let elapsed = last_render_time.elapsed();
                if elapsed < frame_duration {
                    std::thread::sleep(frame_duration - elapsed);
                }
            }
            *last_render_time = Instant::now();

            return Ok(());
        }

        // Swapchain needs to be recreated
        world.remove_resource::<Swapchain>();
    }

    // Check if we can create a swapchain
    let window_size = indices.window(world).size();
    if window_size != Vector2u::ZERO {
        world.insert_permanent_resource(Swapchain::new(surface, window_size, device)?);
    }

    // Sleep for a short duration to limit background FPS
    let frame_duration = Duration::from_secs_f32(1.0 / min_frame_rate);
    let elapsed = last_render_time.elapsed();
    if elapsed < frame_duration {
        std::thread::sleep(frame_duration - elapsed);
    }
    *last_render_time = Instant::now();

    Ok(())
}

/// Runs the needed render phases and returns if the swapchain is still valid
fn render_frame<Game: crate::Game>(
    world: &mut World,
    indices: &RenderIndices<Game>,
) -> Result<bool> {
    let frame_index = indices.frame_index(world).clone();

    // Wait for copy fence before copying data to host visible buffers
    indices.copy_fence_mut(world).wait(&frame_index)?;
    world.run(PreRender)?;

    // Wait for draw fence before setting up frame graph
    indices.draw_fence_mut(world).wait(&frame_index)?;
    world.run(FrameGraphSetup)?;

    if !indices.swapchain(world).is_valid() {
        return Ok(false);
    }

    // Run the main frame graph
    world.run(FrameGraph)?;
    world.run(PostRender)?;

    Ok(!indices.swapchain(world).is_suboptimal())
}
