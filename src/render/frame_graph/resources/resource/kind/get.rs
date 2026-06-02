use crate::render::frame_graph::resources::resource::FrameGraphResourceKind;
use alexandria::gpu::VulkanImageView;

impl<'a> FrameGraphResourceKind<'a> {
    /// Get the image view of the resource
    pub const fn image_view(&self) -> &VulkanImageView {
        match self {
            FrameGraphResourceKind::Transient(image_view) => image_view,
            FrameGraphResourceKind::External(image_view) => *image_view,
        }
    }
}
