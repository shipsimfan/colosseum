use crate::render::Samplers;
use alexandria::gpu::VulkanSampler;
use std::sync::Arc;

impl Samplers {
    /// Get the linear sampler without anisotropic filtering
    pub(in crate::render) fn linear(&self) -> &Arc<VulkanSampler> {
        &self.linear
    }
}
