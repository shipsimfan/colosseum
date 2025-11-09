use crate::graphics::CameraInner;

impl CameraInner {
    /// Called when the window resizes
    pub(in crate::graphics) fn resize(&mut self) {
        self.projection_dirty = true;
        self.viewport_dirty = true;
    }
}
