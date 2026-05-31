use crate::render::FrameContext;
use alexandria::math::{Color4f, Srgb, Vector2i};

impl<'frame, 'surface> FrameContext<'frame, 'surface> {
    /// Begin rendering to the current swapchain image
    pub fn begin_rendering_swapchain(&mut self, clear_color: Color4f<Srgb>) {
        self.data.cmd_begin_rendering(
            self.image_view,
            Vector2i::new(self.size.x as _, self.size.y as _),
            clear_color,
        );
    }
}
