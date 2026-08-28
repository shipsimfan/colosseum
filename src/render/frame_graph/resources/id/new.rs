use crate::render::frame_graph::FrameGraphResourceId;

impl FrameGraphResourceId {
    /// Create a new [`FrameGraphResourceId`] for an external resource, such as the swapchain image
    pub(in crate::render::frame_graph::resources) const fn new_external(
        id: usize,
    ) -> FrameGraphResourceId {
        debug_assert!(
            id < FrameGraphResourceId::TYPE_MASK,
            "external resource ID too large"
        );

        FrameGraphResourceId {
            id: id | FrameGraphResourceId::EXTERNAL,
        }
    }

    /// Create a new [`FrameGraphResourceId`] for a transient resource at render scale
    pub(in crate::render::frame_graph::resources) const fn new_transient_render_scale(
        id: usize,
    ) -> FrameGraphResourceId {
        debug_assert!(
            id < FrameGraphResourceId::TYPE_MASK,
            "render scale transient resource ID too large"
        );

        FrameGraphResourceId {
            id: id | FrameGraphResourceId::TRANSIENT_RENDER_SCALE,
        }
    }

    /// Create a new [`FrameGraphResourceId`] for a transient resource at native scale
    pub(in crate::render::frame_graph::resources) const fn new_transient_native_scale(
        id: usize,
    ) -> FrameGraphResourceId {
        debug_assert!(
            id < FrameGraphResourceId::TYPE_MASK,
            "native scale transient resource ID too large"
        );

        FrameGraphResourceId {
            id: id | FrameGraphResourceId::TRANSIENT_NATIVE_SCALE,
        }
    }
}
