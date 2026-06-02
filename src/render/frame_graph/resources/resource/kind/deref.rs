use crate::render::frame_graph::resources::resource::FrameGraphResourceKind;
use alexandria::gpu::VulkanImageView;
use std::ops::Deref;

impl<'a> Deref for FrameGraphResourceKind<'a> {
    type Target = VulkanImageView;

    fn deref(&self) -> &Self::Target {
        self.image_view()
    }
}

impl<'a> AsRef<VulkanImageView> for FrameGraphResourceKind<'a> {
    fn as_ref(&self) -> &VulkanImageView {
        self.image_view()
    }
}
