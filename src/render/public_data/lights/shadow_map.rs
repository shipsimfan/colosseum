use crate::{
    Result,
    render::{FrameGraphResourceId, GraphicsDevice, Shader},
};
use alexandria::{
    gpu::VulkanPipelineShaderStageCreateInfo,
    math::{Color3f, Srgb},
};

/// The definitions required by the shadow map frame graph node to render a light type's shadow
/// maps
pub(in crate::render) trait ShadowMapLight: 'static + Sized {
    /// The name of the light type
    const NAME: &'static str;

    /// The color for the render pass
    const COLOR: Color3f<Srgb>;

    /// The number of cascades for the shadow map
    const CASCADES: u32;

    /// The ID of the shadow map resource in the frame graph
    const SHADOW_MAP: FrameGraphResourceId;

    /// The GPU type of the light
    type Gpu;

    /// The type of the descriptor set layout used for the shadow map
    type DescriptorSetLayout: 'static;

    /// Creates the shader for the shadow map rendering of this light type
    fn create_shader(device: &GraphicsDevice) -> Result<Shader>;

    /// Get the shader stages for the shadow map rendering of this light type
    fn shader_stages<'a>(shader: &'a Shader) -> Vec<VulkanPipelineShaderStageCreateInfo<'a>>;
}
