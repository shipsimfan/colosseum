use crate::render::frame_graph::{FrameGraphResourceBuilder, FrameGraphResourceId};

impl<'a> FrameGraphResourceBuilder<'a> {
    /// Mark that a resource will be used as a color attachment
    pub fn set_color(&mut self, id: FrameGraphResourceId) {
        if id.is_transient_render_scale() {
            self.transient_render_scale[id.index()].set_color();
        } else if id.is_transient_native_scale() {
            todo!("transient native scale resources are not yet implemented")
        } else if id.is_transient_static_scale() {
            todo!("transient static resources are not yet implemented")
        }

        // Do nothing for external resources
    }

    /// Mark that a resource will be used as a depth attachment
    pub fn set_depth(&mut self, id: FrameGraphResourceId) {
        if id.is_transient_render_scale() {
            self.transient_render_scale[id.index()].set_depth();
        } else if id.is_transient_native_scale() {
            todo!("transient native scale resources are not yet implemented")
        } else if id.is_transient_static_scale() {
            todo!("transient static resources are not yet implemented")
        }

        // Do nothing for external resources
    }
}
