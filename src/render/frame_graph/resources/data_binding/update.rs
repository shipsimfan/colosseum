use crate::render::{
    DescriptorSetBinding, FrameIndex, GpuDataBuffers, GraphicsDevice, MAX_FRAMES_IN_FLIGHT,
    frame_graph::resources::FrameGraphDataBinding,
};
use alexandria::gpu::{VulkanDescriptorBufferInfo, VulkanDescriptorSet, VulkanWriteDescriptorSet};

impl FrameGraphDataBinding {
    /// Update the data binding with the new buffer and frame index
    pub fn update(
        &mut self,
        data_buffers: &GpuDataBuffers,
        descriptor_sets: &[(
            [VulkanDescriptorSet; MAX_FRAMES_IN_FLIGHT],
            &'static [DescriptorSetBinding],
        )],
        frame_index: &FrameIndex,
        device: &GraphicsDevice,
    ) {
        let (epoch, buffer, capacity) = data_buffers.get(self.index, frame_index);

        // Check if the epoch for the current frame index matches the provided epoch
        if epoch == self.epoch[**frame_index] {
            return;
        }

        // Update the descriptor sets and bindings with the new buffer if necessary
        for &(descriptor_set, binding) in &self.bindings {
            let descriptor_set_info = &descriptor_sets[descriptor_set];
            let descriptor_set = &descriptor_set_info.0[**frame_index];
            let descriptor_type = descriptor_set_info.1[binding as usize].r#type();

            device.update_descriptor_sets(
                &[VulkanWriteDescriptorSet::new(
                    descriptor_set,
                    binding,
                    0,
                    descriptor_type,
                    &[],
                    &[VulkanDescriptorBufferInfo::new(buffer, 0, capacity)],
                )],
                &[],
            );
        }

        // Update the epoch for the current frame index
        self.epoch[**frame_index] = epoch;
    }
}
