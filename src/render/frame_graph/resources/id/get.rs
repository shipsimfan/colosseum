use crate::render::frame_graph::FrameGraphResourceId;

impl FrameGraphResourceId {
    /// Get the index of the resource
    pub const fn index(&self) -> usize {
        self.id & !FrameGraphResourceId::TYPE_MASK
    }

    /// Is this an external resource (e.g., from the swapchain)?
    pub const fn is_external(&self) -> bool {
        (self.id & FrameGraphResourceId::TYPE_MASK) == FrameGraphResourceId::EXTERNAL
    }

    /// Is this a transient resource at render scale?
    pub const fn is_transient_render_scale(&self) -> bool {
        (self.id & FrameGraphResourceId::TYPE_MASK) == FrameGraphResourceId::TRANSIENT_RENDER_SCALE
    }

    /// Is this a transient resource at native scale?
    pub const fn is_transient_native_scale(&self) -> bool {
        (self.id & FrameGraphResourceId::TYPE_MASK) == FrameGraphResourceId::TRANSIENT_NATIVE_SCALE
    }
}
