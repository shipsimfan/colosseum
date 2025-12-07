use crate::{Result, graphics::Material};
use win32::d3d11::ID3D11DeviceContext;

impl Material {
    /// Binds the material properties for rendering
    pub fn bind(&mut self, device_context: &mut ID3D11DeviceContext) -> Result<()> {
        self.shader.bind(device_context);
        self.buffer.bind(device_context)
    }
}
