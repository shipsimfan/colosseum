use crate::render::frame_graph::builder::FrameGraphBuilderResource;
use alexandria::gpu::{VulkanAttachmentLoadOp, VulkanSampler};
use std::sync::Arc;

impl<'a> FrameGraphBuilderResource<'a> {
    /// Set that the frame graph resource is being used as a color attachment
    pub fn set_color(&mut self) -> VulkanAttachmentLoadOp {
        match self {
            FrameGraphBuilderResource::External(external) => external.set_color(),
            FrameGraphBuilderResource::Transient(transient) => transient.set_color(),
        }
    }

    /// Set that the frame graph resource is being used as a sampled image
    pub fn set_sampled_image(
        &mut self,
        descriptor_set: usize,
        binding: u32,
        sampler: &Arc<VulkanSampler>,
    ) {
        match self {
            FrameGraphBuilderResource::External(_) => {
                panic!("unable to sample external resources")
            }
            FrameGraphBuilderResource::Transient(transient) => {
                transient.set_sampled_image(descriptor_set, binding, sampler)
            }
        }
    }
}
