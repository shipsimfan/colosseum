use crate::render::Shader;
use alexandria::gpu::VulkanPipeline;

mod bind;
mod new;

/// A shader for rendering lit forward materials
pub struct LitForwardShader {
    /// The pipeline for the shader
    pipeline: VulkanPipeline,

    /// The underlying shader object
    #[allow(unused)]
    shader: Shader,
}
