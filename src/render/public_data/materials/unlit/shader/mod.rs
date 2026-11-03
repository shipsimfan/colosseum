use crate::render::Shader;
use alexandria::gpu::VulkanPipeline;

mod bind;
mod new;

/// A shader for unlit materials
pub struct UnlitShader {
    /// The pipeline for the shader
    pipeline: VulkanPipeline,

    /// The underlying shader object
    #[allow(unused)]
    shader: Shader,
}
