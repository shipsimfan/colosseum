use crate::render::job::RenderToken;

impl RenderToken {
    /// Create a new [`RenderToken`]
    pub(in crate::render::job::graphics_device) fn new(frame_index: usize) -> RenderToken {
        RenderToken { frame_index }
    }
}
