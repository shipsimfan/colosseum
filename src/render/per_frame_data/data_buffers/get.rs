use crate::render::{FrameIndex, GpuDataBuffers};
use alexandria::gpu::VulkanBuffer;
use std::any::TypeId;

impl GpuDataBuffers {
    /// Get an iterator over the data buffer types
    pub fn types(&self) -> impl Iterator<Item = TypeId> {
        self.buffers.iter().map(|buffer| buffer.r#type())
    }

    /// Get the epoch, buffer, and capacity for the specified data buffer and frame index
    pub fn get(&self, index: usize, frame_index: &FrameIndex) -> (u64, &VulkanBuffer, u64) {
        self.buffers[index].epoch_and_buffer(frame_index)
    }
}
