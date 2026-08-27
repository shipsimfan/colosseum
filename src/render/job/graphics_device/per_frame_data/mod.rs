use crate::render::{FrameGraphTransientBuffer, RenderData};
use alexandria::{Id, gpu::VulkanCommandBuffer};

mod get;
mod new;

/// Data that is needed for each frame in flight
pub(in crate::render::job::graphics_device) struct PerFrameData {
    /// The command buffers that have been allocated in the pool
    command_buffer: Id<VulkanCommandBuffer>,

    /// The transient buffers that have been allocated for temporary data during rendering
    transient_buffer: FrameGraphTransientBuffer,

    /// The render data for each frame
    render_data: RenderData,
}
