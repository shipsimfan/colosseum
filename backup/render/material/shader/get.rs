use crate::render::Shader;
use alexandria::gpu::VulkanShaderModule;

impl Shader {
    /// Get the underlying [`VulkanShaderModule`] for this [`Shader`]
    pub(in crate::render) fn module(&self) -> &VulkanShaderModule {
        &self.module
    }
}
