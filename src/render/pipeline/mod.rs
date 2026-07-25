use crate::render::Shader;
use alexandria::gpu::{VulkanPipeline, VulkanPipelineLayout};
use std::sync::Arc;

/// A pipeline and layout that can be used for rendering objects
pub(in crate::render) struct Pipeline {
    /// The pipeline itself
    pipeline: VulkanPipeline,

    /// The pipeline layout used to create the pipeline
    layout: VulkanPipelineLayout,

    /// The shaders being used by this pipeline
    #[allow(unused)]
    shader: Vec<Arc<Shader>>,
}
