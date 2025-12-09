use crate::{
    Result,
    graphics::{AntiAliasing, Camera},
    math::Vector2u,
};
use win32::d3d11::{ID3D11Device, ID3D11DeviceContext};

impl Camera {
    /// Set this camera as active, updating if needed
    pub(in crate::graphics) fn bind(
        &mut self,
        window_size: Vector2u,
        render_scale: f32,
        anti_aliasing: Option<AntiAliasing>,
        clear_color: [f32; 4],
        device: &ID3D11Device,
        device_context: &mut ID3D11DeviceContext,
    ) -> Result<()> {
        // Update transform
        let epoch = self.transform.update_camera();
        let epoch_changed = epoch != self.transform_epoch;
        self.transform_epoch = epoch;

        let update_needed = epoch_changed || self.projection_dirty;

        // Update projection matrix
        if self.projection_dirty {
            self.projection_matrix = self.projection.matrix(window_size);
            self.projection_dirty = false;
        }

        // Update constant buffer if needed
        if update_needed {
            self.buffer.view = self.transform.matrix() * self.projection_matrix;
            self.buffer.position = self.transform.position();
        }

        // Update viewport if needed
        if self.viewport_dirty {
            self.post_processing.resize(
                window_size,
                &self.relative_viewport,
                render_scale,
                anti_aliasing,
                device,
            )?;

            self.viewport_dirty = false;
        }

        // Set active
        self.buffer.bind(device_context)?;
        self.post_processing.clear(clear_color, device_context);
        self.post_processing.bind_main_color_output(device_context);

        Ok(())
    }
}
