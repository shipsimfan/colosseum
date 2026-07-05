use alexandria::gpu::VulkanShaderModule;

mod get;
mod new;

/// A shader being used in rendering
pub struct Shader {
    /// The compiled shader module
    module: VulkanShaderModule,
}
