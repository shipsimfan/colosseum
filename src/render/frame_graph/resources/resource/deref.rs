use crate::render::frame_graph::FrameGraphResource;
use alexandria::gpu::VulkanImageView;
use std::ops::Deref;

impl<'a> Deref for FrameGraphResource<'a> {
    type Target = VulkanImageView;

    fn deref(&self) -> &Self::Target {
        self.image_view()
    }
}

impl<'a> AsRef<VulkanImageView> for FrameGraphResource<'a> {
    fn as_ref(&self) -> &VulkanImageView {
        self.image_view()
    }
}
