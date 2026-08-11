use crate::render::job::RenderToken;

impl RenderToken {
    /// Get the frame index associated with this render token
    pub(in crate::render::job::graphics_device) fn frame_index(self) -> usize {
        self.frame_index
    }
}
