use crate::graphics::managed_objects::material::MaterialShader;
use win32::d3d11::ID3D11DeviceContext;

impl MaterialShader {
    /// Set this shader as the active shader for rendering
    pub(in crate::graphics) fn bind(&self, device_context: &mut ID3D11DeviceContext) {
        self.vertex_shader.bind(device_context);
        self.pixel_shader.bind(device_context);
    }
}
