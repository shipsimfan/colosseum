use crate::render::Pipeline;
use alexandria::gpu::{VulkanDescriptorSetLayout, VulkanPipelineLayout};

mod get;
mod new;

/// The render objects that are created once and don't change
pub(crate) struct FixedRenderObjects {
    /// The descriptor set layout for camera data
    camera_data_layout: VulkanDescriptorSetLayout,

    /// The pipeline layout for unlit opaque rendering
    unlit_forward_pipeline_layout: VulkanPipelineLayout,

    /// The set of pipelines created for frame graph nodes that don't use materials
    pipelines: Vec<Pipeline>,
}
