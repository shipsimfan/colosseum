use crate::graphics::PostProcessing;
use win32::d3d11::ID3D11DeviceContext;

impl PostProcessing {
    /// Clear the main color output
    pub(in crate::graphics::managed_objects::camera) fn clear(
        &mut self,
        color: [f32; 4],
        device_context: &mut ID3D11DeviceContext,
    ) {
        self.render_scale_objects.clear(color, device_context);
    }
}
