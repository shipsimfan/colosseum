//! The per-frame objects

use crate::{Result, render::GraphicsDevice};
use colosseum_ecs::World;

mod acquire_image_semaphore;
mod draw_fence;
mod render_command_buffer;
mod render_complete_semaphore;
mod secondary_command_buffer;

pub(in crate::render) use acquire_image_semaphore::*;
pub(in crate::render) use draw_fence::*;
pub(in crate::render) use render_command_buffer::*;
pub(in crate::render) use render_complete_semaphore::*;
pub(in crate::render) use secondary_command_buffer::*;

/// Initialize the per-frame render data resources
pub(in crate::render) fn create_per_frame_render_data(
    world: &mut World,
    graphics_device: &GraphicsDevice,
) -> Result<()> {
    world.insert_permanent_resource(DrawFence::new(&graphics_device)?);
    world.insert_permanent_resource(AcquireImageSemaphore::new(&graphics_device)?);
    world.insert_permanent_resource(RenderCompleteSemaphore::new(&graphics_device)?);
    world.insert_permanent_resource(RenderCommandBuffer::new(&graphics_device)?);

    Ok(())
}
