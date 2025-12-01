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
        let num_point_lights = self.point_lights.bind(device, device_context)?;
        let num_spot_lights = self.spot_lights.bind(device, device_context)?;
        self.constant_buffer.bind(
            num_directional_lights,
            num_point_lights,
            num_spot_lights,
            device_context,
        )
    }
}
