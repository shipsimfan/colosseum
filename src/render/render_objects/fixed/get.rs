use crate::render::{FixedRenderObjects, MaterialKind, Pipeline, Shader};
use alexandria::gpu::{VulkanDescriptorSetLayout, VulkanPipelineLayout, VulkanSampler};
use std::sync::Arc;

impl FixedRenderObjects {
    /// Get the pipeline layout for a [`MaterialKind`]
    pub fn material_pipeline_layout(&self, kind: MaterialKind) -> &VulkanPipelineLayout {
        &self.pipeline_layouts[match kind {
            MaterialKind::UnlitOpaque => FixedRenderObjects::UNLIT_OPAQUE_PIPELINE_LAYOUT,
        }]
    }

    /// Get a reference to a pipeline
    pub(in crate::render) fn pipeline(&self, index: usize) -> &Pipeline {
        &self.pipelines[index]
    }

    /// Get a reference to a sampler
    pub(in crate::render) fn sampler(&self, index: usize) -> &VulkanSampler {
        &self.samplers[index]
    }

    /// Get a reference to a descriptor set layout
    pub(in crate::render) fn descriptor_set_layout(
        &self,
        index: usize,
    ) -> &VulkanDescriptorSetLayout {
        &self.descriptor_set_layouts[index]
    }

    /// Get a reference to the fullscreen quad shader
    pub(in crate::render) fn fullscreen_quad(&self) -> &Arc<Shader> {
        &self.fullscreen_quad
    }
}
