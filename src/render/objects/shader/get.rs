use crate::render::Shader;
use alexandria::gpu::VulkanShaderModule;

impl Shader {
    /// Get the underlying [`VulkanShaderModule`] for this [`Shader`]
    pub fn module(&self) -> &VulkanShaderModule {
        &self.module
    }
}
