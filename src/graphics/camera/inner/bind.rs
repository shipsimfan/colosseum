use crate::{Error, Result, graphics::CameraInner, math::Vector2u};
use win32::{
    d3d11::{D3D11_MAP, D3D11_MAPPED_SUBRESOURCE, ID3D11DeviceContext},
    try_hresult,
};

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
            let mut mapped_resource = D3D11_MAPPED_SUBRESOURCE::default();
            try_hresult!(device_context.map(
                self.buffer.as_mut(),
                0,
                D3D11_MAP::WriteDiscard,
                0,
                &mut mapped_resource,
            ))
            .map_err(|error| Error::new_inner("unable to map camera constant buffer", error))?;

            self.buffer_content.view = self.transform.matrix() * self.projection_matrix;
            self.buffer_content.position = self.transform.position();
            *unsafe { &mut *(mapped_resource.data as *mut _) } = self.buffer_content;

            device_context.unmap(self.buffer.as_mut(), 0);
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
        let buffer = self.buffer.as_mut() as *mut _;
        device_context.vs_set_constant_buffers(0, 1, &buffer);
        device_context.ps_set_constant_buffers(0, 1, &buffer);
        device_context.rs_set_viewports(1, &self.screen_viewport);

        Ok(())
    }
}
