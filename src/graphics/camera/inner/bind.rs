use crate::{Result, graphics::CameraInner, math::Vector2u};
use win32::d3d11::ID3D11DeviceContext;

impl CameraInner {
    /// Set this camera as active, updating if needed
    pub(in crate::graphics) fn bind(
        &mut self,
        screen_size: Vector2u,
        device_context: &mut ID3D11DeviceContext,
    ) -> Result<()> {
        // Update transform
        let update_needed = self.transform.update_camera() || self.projection_dirty;

        // Update projection matrix
        if self.projection_dirty {
            self.projection_matrix = self.projection.matrix(screen_size);
            self.projection_dirty = false;
        }

        // Update constant buffer if needed
        if update_needed {
            self.buffer.view = self.transform.matrix() * self.projection_matrix;
            self.buffer.position = self.transform.position();
        }

        // Update viewport if needed
        if self.viewport_dirty {
            self.screen_viewport.top_left_x =
                self.relative_viewport.top_left_x * screen_size.x as f32;
            self.screen_viewport.top_left_y =
                self.relative_viewport.top_left_y * screen_size.y as f32;
            self.screen_viewport.width = self.relative_viewport.width * screen_size.x as f32;
            self.screen_viewport.height = self.relative_viewport.height * screen_size.y as f32;

            self.viewport_dirty = false;
        }

        // Set active
        self.buffer.bind(device_context)?;
        device_context.rs_set_viewports(1, &self.screen_viewport);

        Ok(())
    }
}
