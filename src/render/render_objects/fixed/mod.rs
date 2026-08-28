use crate::render::{Pipeline, Shader};
use alexandria::gpu::{
    VulkanDescriptorPoolSize, VulkanDescriptorSetLayout, VulkanPipelineLayout, VulkanSampler,
};
use std::sync::Arc;

mod add;
mod create_descriptor_pool;
mod get;
mod new;

/// The render objects that are created once and don't change
pub(crate) struct FixedRenderObjects {
    /// The pipeline layouts created by frame graph nodes
    pipeline_layouts: Vec<VulkanPipelineLayout>,

    /// The set of pipelines created for frame graph nodes that don't use materials
    pipelines: Vec<Pipeline>,

    /// The samplers created for frame graph nodes
    samplers: Vec<VulkanSampler>,

    /// The descriptor set layouts created for frame graph nodes, and if they are doubled or not
    descriptor_set_layouts: Vec<VulkanDescriptorSetLayout>,

    /// The maximum number of sets that will be created for a given frame
    max_descriptor_sets: u32,

    /// The pool sizes required for descriptors sets in a single frame
    descriptor_pool_sizes: Vec<VulkanDescriptorPoolSize>,

    /// The fullscreen quad shader
    fullscreen_quad: Arc<Shader>,
}

impl FixedRenderObjects {
    /** Pipeline Layouts **/
    pub const UNLIT_OPAQUE_PIPELINE_LAYOUT: usize = 0;

    /** Pipelines **/
    pub const SOLID_COLOR_SKY_PIPELINE: usize = 0;
    pub const TONE_MAP_PIPELINE: usize = 1;
    pub const QUANTIZATION_PIPELINE: usize = 2;
    pub const FXAA_PIPELINE: usize = 3;

    /** Samplers **/
    pub const LINEAR_CLAMP_SAMPLER: usize = 0;

    /** Descriptor Set Layouts **/
    pub const CAMERA_DATA_DESCRIPTOR_SET_LAYOUT: usize = 0;
    pub const POST_PROCESS_DESCRIPTOR_SET_LAYOUT: usize = 1;
}
