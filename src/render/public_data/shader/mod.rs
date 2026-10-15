use alexandria::gpu::VulkanShaderModule;

mod kind;

mod get;
mod new;

pub use kind::*;

/// A shader which can be used in rendering
pub struct Shader {
    /// The compiled shader module
    module: VulkanShaderModule,

    /// The kind of shader this represents
    kind: ShaderKind,
}
