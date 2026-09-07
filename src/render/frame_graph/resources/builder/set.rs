use crate::render::frame_graph::{FrameGraphResourceBuilder, FrameGraphResourceId};

impl<'a> FrameGraphResourceBuilder<'a> {
    /// Mark that a resource will be used as a color attachment
    pub fn set_color(&mut self, id: FrameGraphResourceId) {
        if id.is_transient_render_scale() {
            self.transient_render_scale[id.index()].set_color();
        } else if id.is_transient_native_scale() {
            self.transient_native_scale[id.index()].set_color();
        }

        debug_assert!(
            !id.is_shadow_map(),
            "shadow maps cannot be marked as color attachments"
        );

        // Do nothing for external resources
    }

    /// Mark that a resource will be used as a depth attachment
    pub fn set_depth(&mut self, id: FrameGraphResourceId) {
        if id.is_transient_render_scale() {
            self.transient_render_scale[id.index()].set_depth();
        } else if id.is_transient_native_scale() {
            self.transient_native_scale[id.index()].set_depth();
        }

        // Do nothing for external resources or shadow maps
    }

    /// Mark that a resource will be used as a transfer destination
    pub fn set_transfer_dst(&mut self, id: FrameGraphResourceId) {
        if id.is_transient_render_scale() {
            self.transient_render_scale[id.index()].set_transfer_dst();
        } else if id.is_transient_native_scale() {
            self.transient_native_scale[id.index()].set_transfer_dst();
        }

        debug_assert!(
            !id.is_shadow_map(),
            "shadow maps cannot be marked as transfer destinations"
        );

        // Do nothing for external resources
    }

    /// Mark that a resource will be used as a transfer source
    pub fn set_transfer_src(&mut self, id: FrameGraphResourceId) {
        if id.is_transient_render_scale() {
            self.transient_render_scale[id.index()].set_transfer_src();
        } else if id.is_transient_native_scale() {
            self.transient_native_scale[id.index()].set_transfer_src();
        }

        debug_assert!(
            !id.is_shadow_map(),
            "shadow maps cannot be marked as transfer sources"
        );

        // Do nothing for external resources
    }

    /// Mark that a resource will be used as a sampled image
    pub fn set_sampled_image(&mut self, id: FrameGraphResourceId) {
        if id.is_transient_render_scale() {
            self.transient_render_scale[id.index()].set_sampled_image();
        } else if id.is_transient_native_scale() {
            self.transient_native_scale[id.index()].set_sampled_image();
        }

        // Do nothing for external resources or shadow maps
    }
}
