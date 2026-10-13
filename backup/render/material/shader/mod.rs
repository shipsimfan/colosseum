use alexandria::gpu::VulkanShaderModule;

mod id;
mod kind;

mod get;
mod new;

pub use id::*;
pub use kind::*;

/// A shader being used in rendering
pub(crate) struct Shader {
    /// The compiled shader module
    module: VulkanShaderModule,
}
