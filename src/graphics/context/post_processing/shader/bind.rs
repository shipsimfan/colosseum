use crate::graphics::PostProcessingShader;
use win32::d3d11::ID3D11DeviceContext;

impl PostProcessingShader {
    /// Bind this shader to be the active one on `device_context`
    pub(in crate::graphics::context::post_processing) fn bind(
        &mut self,
        device_context: &mut ID3D11DeviceContext,
    ) {
        self.pixel_shader.bind(device_context);
    }
}
