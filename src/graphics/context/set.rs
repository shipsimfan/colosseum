use crate::{
    Result,
    graphics::{AntiAliasing, DisplayMode, GraphicsContext},
    math::{Vector2i, Vector2u},
};

impl GraphicsContext {
    /// Set the window's display mode, size, and position
    pub fn set_size_and_position(&mut self, size: Vector2u, position: Vector2i) -> Result<()> {
        let (window_size, position) = self.display_mode.client_to_window(size, position)?;
        self.message_thread
            .set_size_and_position(window_size, position)?;
        self.size = size;
        Ok(())
    }

    /// Set the size of the window
    pub fn set_size(&mut self, size: Vector2u) -> Result<()> {
        self.set_size_and_position(size, self.message_thread.window_position())
    }

    /// Set the position of the window
    pub fn set_position(&mut self, position: Vector2i) -> Result<()> {
        self.set_size_and_position(self.size, position)
    }

    /// Set the mode the window should display as
    pub fn set_display_mode(&mut self, display_mode: DisplayMode) -> Result<()> {
        self.message_thread.set_display_mode(display_mode)?;
        self.display_mode = display_mode;
        Ok(())
    }

    /// Sets if the rendering will be aligned with vertical syncs
    pub fn set_vsync(&mut self, vsync: bool) {
        self.vsync = vsync;
    }

    /// Set the window title
    pub fn set_title(&mut self, title: &str) -> Result<()> {
        let mut title: Vec<_> = title.encode_utf16().collect();
        title.push(0);
        self.message_thread.set_window_title(title)?;
        Ok(())
    }

    /// Set the type of anti-aliasing being used
    pub fn set_anti_aliasing(&mut self, anti_aliasing: Option<AntiAliasing>) {
        self.anti_aliasing = anti_aliasing;
        self.render_scale_dirty = true;
    }

    /// Set the scale internal rendering occurs at
    pub fn set_render_scale(&mut self, render_scale: f32) {
        self.render_scale = render_scale;
        self.render_scale_dirty = true;
    }

    /// Set the render scale post-process pass to use linear filtering
    pub fn set_render_scale_linear(&mut self) {
        self.post_processing.set_render_scale_linear();
    }

    /// Set the render scale post-process pass to use point filtering
    pub fn set_render_scale_point(&mut self) {
        self.post_processing.set_render_scale_point();
    }
}
