use alexandria::gpu::VulkanSampler;
use std::sync::Arc;

mod state;

/// How a resource is written to by a node
pub(in crate::render::frame_graph) enum FrameGraphResourceUsage<'a> {
    /// The resource is written to as a color attachment
    ColorAttachment,

    /// The resource is written to as a depth attachment
    DepthAttachment,

    /// The resource is written to as a depth attachment, but a render pass shouldn't be
    /// automatically started
    ShadowMapDepthAttachment,

    /// The resource is written to as a transfer destination
    TransferDst,

    /// The resource is read from as a transfer source
    TransferSrc,

    /// The resource is read from as a sampled image
    SampledImage {
        descriptor_set: usize,
        binding: u32,
        sampler: &'a Arc<VulkanSampler>,
    },
}
