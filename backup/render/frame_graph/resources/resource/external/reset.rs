use crate::render::frame_graph::FrameGraphExternalResource;

impl<'a> FrameGraphExternalResource<'a> {
    /// Reset the resource to its initial state for a frame
    pub fn reset(&mut self) {
        unsafe { *self.used.get() = false };
    }
}
