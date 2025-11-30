use crate::{Result, graphics::context::Lights};
use win32::d3d11::{ID3D11Device, ID3D11DeviceContext};

impl Lights {
    /// Bind the light buffers
    pub fn bind(
        &mut self,
        device: &ID3D11Device,
        device_context: &mut ID3D11DeviceContext,
    ) -> Result<()> {
        let num_directional_lights = self.directional_lights.bind(device, device_context)?;
        self.constant_buffer
            .bind(num_directional_lights, device_context)
    }
}
