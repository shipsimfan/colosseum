use crate::render::transfer::{GpuTransferCommand, StagingBuffer};
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

    /// The staging buffer to copy data with
    staging_buffer: StagingBuffer,
}
