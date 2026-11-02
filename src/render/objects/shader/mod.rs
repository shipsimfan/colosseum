use alexandria::gpu::VulkanShaderModule;

mod get;
mod new;

/// A shader which can be used in rendering
pub(in crate::render) struct Shader {
    /// The compiled shader module
    module: VulkanShaderModule,
}
