use crate::render::frame_graph::builder::FrameGraphBuilderResource;
use alexandria::gpu::VulkanAttachmentLoadOp;

impl<'a> FrameGraphBuilderResource<'a> {
    /// Set that the frame graph resource is being used as a color attachment
    pub fn set_color(&mut self) -> VulkanAttachmentLoadOp {
        match self {
            FrameGraphBuilderResource::External(external) => external.set_color(),
            FrameGraphBuilderResource::Transient(transient) => transient.set_color(),
        }
    }
}
