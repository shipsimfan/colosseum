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
    pub const LIT_OPAQUE_PIPELINE_LAYOUT: usize = 1;

    /** Pipelines **/
    pub const SOLID_COLOR_SKY_PIPELINE: usize = 0;
    pub const PROCEDURAL_SKY_PIPELINE: usize = 1;
    pub const TONE_MAP_PIPELINE: usize = 2;
    pub const QUANTIZATION_PIPELINE: usize = 3;
    pub const FXAA_PIPELINE: usize = 4;

    /** Samplers **/
    pub const LINEAR_CLAMP_SAMPLER: usize = 0;

    /** Descriptor Set Layouts **/
    pub const CAMERA_DESCRIPTOR_SET_LAYOUT: usize = 0;
    pub const RENDERABLES_DESCRIPTOR_SET_LAYOUT: usize = 1;
    pub const LIGHTING_DESCRIPTOR_SET_LAYOUT: usize = 2;
    pub const POST_PROCESS_DESCRIPTOR_SET_LAYOUT: usize = 3;

    /** Descriptor Sets **/
    pub const CAMERA_DESCRIPTOR_SET: usize = 0;
    pub const RENDERABLES_DESCRIPTOR_SET: usize = 1;
    pub const LIGHTING_DESCRIPTOR_SET: usize = 2;

    pub const TONE_MAP_DESCRIPTOR_SET: usize = 3;
    pub const QUANTIZATION_DESCRIPTOR_SET: usize = 4;
    pub const FXAA_DESCRIPTOR_SET: usize = 5;

    /** Device Buffers **/
    pub const CAMERA_DEVICE_BUFFER: usize = 0;
    pub const RENDERABLES_DEVICE_BUFFER: usize = 1;
    pub const LIGHTING_METADATA_DEVICE_BUFFER: usize = 2;
    pub const DIRECTIONAL_LIGHTS_DEVICE_BUFFER: usize = 3;
    pub const POINT_LIGHTS_DEVICE_BUFFER: usize = 4;
    pub const SPOT_LIGHTS_DEVICE_BUFFER: usize = 5;
}
