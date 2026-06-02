use crate::render::frame_graph::FrameGraphResourceId;

impl FrameGraphResourceId {
    /// Create a new [`FrameGraphResourceId`] for an external resource, such as the swapchain image
    pub(in crate::render::frame_graph::resources) const fn new_external(
        id: usize,
    ) -> FrameGraphResourceId {
        debug_assert!(
            id < FrameGraphResourceId::EXTERNAL_BIT,
            "external resource ID too large"
        );

        FrameGraphResourceId {
            id: id | FrameGraphResourceId::EXTERNAL_BIT,
        }
    }

    /// Create a new [`FrameGraphResourceId`] for a transient resource, which is managed by the
    /// frame graph
    pub(in crate::render::frame_graph::resources) const fn new_transient(
        id: usize,
    ) -> FrameGraphResourceId {
        debug_assert!(
            id < FrameGraphResourceId::EXTERNAL_BIT,
            "transient resource ID too large"
        );

        FrameGraphResourceId { id }
    }
}
