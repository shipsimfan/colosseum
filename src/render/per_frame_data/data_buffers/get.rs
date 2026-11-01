use crate::render::{FrameIndex, GpuDataBuffers};
use alexandria::gpu::VulkanBuffer;
use std::any::TypeId;

impl GpuDataBuffers {
    /// Get the number of data buffers
    pub fn len(&self) -> usize {
        self.buffers.len()
    }

    /// Get an iterator over the data buffer types
    pub fn types(&self) -> impl Iterator<Item = TypeId> {
        self.buffers.iter().map(|buffer| buffer.r#type())
    }

    /// Get an iterator over the data buffers
    pub fn buffers(
        &self,
        frame_index: &FrameIndex,
    ) -> impl Iterator<Item = (u64, &VulkanBuffer, u64)> {
        self.buffers
            .iter()
            .map(|buffer| buffer.epoch_and_buffer(frame_index))
    }
}
