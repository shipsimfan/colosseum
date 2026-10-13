use alexandria::gpu::{VulkanImageMemoryBarrier, VulkanRenderingAttachmentInfo};

/// An element that can safely have an empty vec transmuted to a different lifetime
pub(in crate::render::frame_graph) trait Arenable {
    /// The type of the elements stored in the arena
    type T<'a>;

    /// Transmute an empty vec to a different lifetime
    unsafe fn transmute<'a>(vec: &'a mut Vec<Self::T<'static>>) -> &'a mut Vec<Self::T<'a>> {
        unsafe { std::mem::transmute(vec) }
    }
}

impl Arenable for VulkanImageMemoryBarrier<'_> {
    type T<'a> = VulkanImageMemoryBarrier<'a>;
}

impl Arenable for VulkanRenderingAttachmentInfo<'_> {
    type T<'a> = VulkanRenderingAttachmentInfo<'a>;
}
