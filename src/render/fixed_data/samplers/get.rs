use crate::render::Samplers;
use alexandria::gpu::VulkanSampler;
use std::sync::Arc;

impl Samplers {
    /// Get the linear sampler without anisotropic filtering
    pub fn linear(&self) -> &Arc<VulkanSampler> {
        &self.linear
    }

    /// Get the percentage-closer filtering (PCF) sampler
    pub fn pcf(&self) -> &Arc<VulkanSampler> {
        &self.pcf
    }
}
