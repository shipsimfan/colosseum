use crate::render::frame_graph::FrameGraphTransientResource;

impl FrameGraphTransientResource {
    /// Reset the resource to its initial state for a frame
    pub fn reset(&mut self) {
        unsafe { *self.used.get() = false };
    }
}
