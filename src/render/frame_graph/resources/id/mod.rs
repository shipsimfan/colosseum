mod get;
mod new;

/// An ID identifying a resource in the frame graph
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(in crate::render::frame_graph) struct FrameGraphResourceId {
    /// The ID of the resource
    ///
    /// The top bit of the ID is used to indicate if this is a transient or external resource. If
    /// the top bit is set, this is an external resource. Otherwise, this is a transient resource.
    id: usize,
}

impl FrameGraphResourceId {
    /// The bit used to indicate that a resource ID is external (e.g., from the swapchain)
    const EXTERNAL_BIT: usize = 1 << (usize::BITS - 1);

    /// The ID for the swapchain image, which is always an external resource with ID 0
    pub const SWAPCHAIN_IMAGE: FrameGraphResourceId = FrameGraphResourceId::new_external(0);
}
