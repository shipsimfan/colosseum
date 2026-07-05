use crate::render::RenderMaterial;
use alexandria::gpu::VulkanPipeline;

impl RenderMaterial {
    /// Get the pipeline associated with this material
    pub(in crate::render) fn pipeline(&self) -> &VulkanPipeline {
        &self.pipeline
    }
}
