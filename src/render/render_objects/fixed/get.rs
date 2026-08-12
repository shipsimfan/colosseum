use crate::render::{FixedRenderObjects, MaterialKind, Pipeline};
use alexandria::gpu::{VulkanDescriptorSetLayout, VulkanPipelineLayout};

impl FixedRenderObjects {
    /// Get the descriptor set layout for camera data
    pub fn camera_data_layout(&self) -> &VulkanDescriptorSetLayout {
        &self.camera_data_layout
    }

    /// Get the pipeline layout for a [`MaterialKind`]
    pub fn material_pipeline_layout(&self, kind: MaterialKind) -> &VulkanPipelineLayout {
        match kind {
            MaterialKind::UnlitOpaque => &self.unlit_forward_pipeline_layout,
        }
    }

    /// Get the list of pipelines created for frame graph nodes that don't use materials
    pub(in crate::render) fn pipelines(&self) -> &[Pipeline] {
        &self.pipelines
    }
}
