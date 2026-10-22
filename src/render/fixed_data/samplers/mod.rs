use alexandria::gpu::VulkanSampler;
use std::sync::Arc;

mod get;
mod new;

/// The fixed samplers used in the rendering pipeline
pub(in crate::render) struct Samplers {
    /// A linear sampler without anisotropic filtering
    linear: Arc<VulkanSampler>,
}
