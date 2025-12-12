use crate::graphics::context::PostProcessing;

impl PostProcessing {
    /// Set the render scale post-process pass to use linear filtering
    pub fn set_render_scale_linear(&mut self) {
        self.render_scale_point = false;
    }

    /// Set the render scale post-process pass to use point filtering
    pub fn set_render_scale_point(&mut self) {
        self.render_scale_point = true;
    }
}
