use crate::graphics::CameraPostProcessing;
use win32::d3d11::ID3D11DeviceContext;

impl CameraPostProcessing {
    /// Bind the first HDR output texture for rendering
    pub(in crate::graphics::managed_objects::camera) fn bind_main_color_output(
        &mut self,
        device_context: &mut ID3D11DeviceContext,
    ) {
        self.render_scale_objects
            .bind_main_color_output(device_context);
    }
}
