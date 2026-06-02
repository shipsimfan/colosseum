use alexandria::gpu::VulkanImageView;

mod deref;
mod from;
mod get;

/// The kind of a resource in the frame graph, which determines how the resource is created and
/// accessed by nodes in the frame graph
pub(in crate::render::frame_graph::resources::resource) enum FrameGraphResourceKind<'a> {
    /// A transient resource that is created by the frame graph and used as an input or output for
    /// nodes in the frame graph
    Transient(VulkanImageView),

    /// An external resource that is provided to the frame graph, such as the swapchain image
    External(&'a VulkanImageView),
}
