use crate::render::{
    Vertex,
    transfer::{GpuTransferCommand, StagingBuffer},
};
use alexandria::{
    Id,
    gpu::{VulkanCommandBuffer, VulkanCommandPool, VulkanFence},
};
use std::sync::mpsc::Receiver;

mod handle_command;
mod new;

/// The transfer queue as used on the render or transfer thread
pub(in crate::render) struct RenderGpuTransferQueue {
    /// The receiver for transfer commands
    receiver: Receiver<GpuTransferCommand>,

    /// The command pool used for transfer commands
    command_pool: VulkanCommandPool,

    /// The id of the command buffer used for transfer commands
    command_buffer_id: Id<VulkanCommandBuffer>,

    /// The fence used to wait for transfer commands to complete
    fence: VulkanFence,

    /// The staging buffer used for vertex data
    vertex_staging_buffer: StagingBuffer<Vertex>,

    /// The staging buffer used for index data
    index_staging_buffer: StagingBuffer<u32>,
}
