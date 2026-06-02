use crate::render::frame_graph::FrameGraphResourceId;

impl FrameGraphResourceId {
    /// Get the index of the resource
    pub const fn index(&self) -> usize {
        self.id & !FrameGraphResourceId::EXTERNAL_BIT
    }

    /// Is this an external resource (e.g., from the swapchain)?
    pub const fn is_external(&self) -> bool {
        (self.id & FrameGraphResourceId::EXTERNAL_BIT) != 0
    }

    /// Is this a transient resource, which is managed by the frame graph?
    pub const fn is_transient(&self) -> bool {
        !self.is_external()
    }
}
