use crate::render::Pipeline;
use alexandria::gpu::VulkanPipelineLayout;

impl Pipeline {
    /// Get the pipeline layout used to create this pipeline
    pub(in crate::render) fn layout(&self) -> &VulkanPipelineLayout {
        &self.layout
    }
}
