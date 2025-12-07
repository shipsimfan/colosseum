use crate::{
    graphics::{DisplayMode, GraphicsContext},
    math::{Vector2i, Vector2u},
};

impl GraphicsContext {
    /// Get the size of the window
    pub fn size(&self) -> Vector2u {
        self.size
    }

    /// Get the position of the window
    pub fn position(&self) -> Vector2i {
        self.message_thread.window_position()
    }

    /// Get the mode the window should display as
    pub fn display_mode(&self) -> DisplayMode {
        self.display_mode
    }

    /// Get if the rendering will be aligned with vertical syncs
    pub fn vsync(&self) -> bool {
        self.vsync
    }
}
