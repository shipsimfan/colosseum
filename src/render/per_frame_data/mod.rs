//! The per-frame objects

use crate::{Result, render::GraphicsDevice};
use colosseum_ecs::World;
use std::sync::Arc;

mod acquire_image_semaphore;
mod copy_command_buffer;
mod copy_complete_semaphore;
mod copy_fence;
mod data_buffers;
mod draw_fence;
mod render_command_buffer;
mod render_complete_semaphore;
mod secondary_command_buffer;

pub(in crate::render) use acquire_image_semaphore::*;
pub(in crate::render) use copy_command_buffer::*;
pub(in crate::render) use copy_complete_semaphore::*;
pub(in crate::render) use copy_fence::*;
pub(in crate::render) use data_buffers::*;
pub(in crate::render) use draw_fence::*;
pub(in crate::render) use render_command_buffer::*;
pub(in crate::render) use render_complete_semaphore::*;
pub(in crate::render) use secondary_command_buffer::*;

/// Initialize the per-frame render data resources
pub(in crate::render) fn create_per_frame_render_data(
    world: &mut World,
    device: &Arc<GraphicsDevice>,
) -> Result<()> {
    world.insert_permanent_resource(DrawFence::new(device)?);
    world.insert_permanent_resource(AcquireImageSemaphore::new(device)?);
    world.insert_permanent_resource(RenderCompleteSemaphore::new(device)?);
    world.insert_permanent_resource(RenderCommandBuffer::new(device)?);
    world.insert_permanent_resource(CopyFence::new(device)?);
    world.insert_permanent_resource(CopyCompleteSemaphore::new(device)?);
    world.insert_permanent_resource(CopyCommandBuffer::new(device)?);

    GpuDataBuffers::new(device.clone(), world)?;

    Ok(())
}
