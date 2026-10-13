use crate::render::frame_graph::FrameGraphResourceBuilder;

impl<'a> FrameGraphResourceBuilder<'a> {
    /// Clear the transient resource information
    pub fn clear_transient(&mut self) {
        self.transient_render_scale.clear();
        self.transient_native_scale.clear();
    }
}
