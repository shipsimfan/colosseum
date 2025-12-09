use crate::{graphics::Camera, math::Vector2f};
use win32::d3d11::D3D11_VIEWPORT;

impl Camera {
    /// Get the current viewport being used
    pub fn viewport(&self) -> (Vector2f, Vector2f) {
        (
            Vector2f::new(
                self.relative_viewport.top_left_x,
                self.relative_viewport.top_left_y,
            ),
            Vector2f::new(self.relative_viewport.width, self.relative_viewport.height),
        )
    }

    /// Set the viewport being used by the camera
    pub fn set_viewport(&mut self, position: Vector2f, size: Vector2f) {
        self.viewport_dirty = true;
        self.relative_viewport = D3D11_VIEWPORT {
            top_left_x: position.x,
            top_left_y: position.y,
            width: size.x,
            height: size.y,
            ..Default::default()
        };
    }
}
