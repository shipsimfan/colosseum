use crate::render::frame_graph::resources::resource::FrameGraphResourceKind;
use alexandria::gpu::VulkanImageView;

impl<'a> From<VulkanImageView> for FrameGraphResourceKind<'a> {
    fn from(image_view: VulkanImageView) -> Self {
        FrameGraphResourceKind::Transient(image_view)
    }
}

impl<'a> From<&'a VulkanImageView> for FrameGraphResourceKind<'a> {
    fn from(image_view: &'a VulkanImageView) -> Self {
        FrameGraphResourceKind::External(image_view)
    }
}
