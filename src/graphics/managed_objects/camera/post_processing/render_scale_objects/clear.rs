use crate::graphics::managed_objects::camera::post_processing::RenderScaleObjects;
use win32::d3d11::ID3D11DeviceContext;

impl RenderScaleObjects {
    /// Clear the main color output
    pub(in crate::graphics::managed_objects::camera) fn clear(
        &mut self,
        color: [f32; 4],
        device_context: &mut ID3D11DeviceContext,
    ) {
        self.hdr_output1.clear(color, device_context);
        self.depth_buffer.clear(1.0, device_context);
    }
}
