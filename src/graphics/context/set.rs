use crate::{
    Result,
    graphics::{DisplayMode, GraphicsContext},
    math::{Color3f, Vector2i, Vector2u},
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

    /// Set the ambient light color
    pub fn set_ambient_color(&mut self, ambient_color: Color3f) {
        self.lights.set_ambient_color(ambient_color);
    }

    /// Set the intensity of the ambient light
    pub fn set_ambient_intensity(&mut self, ambient_intensity: f32) {
        self.lights.set_ambient_intensity(ambient_intensity);
    }
}
