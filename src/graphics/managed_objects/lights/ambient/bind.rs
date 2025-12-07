use crate::{Result, graphics::AmbientLight};
use win32::d3d11::ID3D11DeviceContext;

impl AmbientLight {
    /// Bind the light buffers
    pub(in crate::graphics::managed_objects::lights) fn bind(
        &mut self,
        num_directional_lights: Option<u32>,
        num_point_lights: Option<u32>,
        num_spot_lights: Option<u32>,
        device_context: &mut ID3D11DeviceContext,
    ) -> Result<()> {
        if let Some(num_directional_lights) = num_directional_lights {
            self.constant_buffer.num_directional_lights = num_directional_lights;
        }

        if let Some(num_point_lights) = num_point_lights {
            self.constant_buffer.num_point_lights = num_point_lights;
        }

        if let Some(num_spot_lights) = num_spot_lights {
            self.constant_buffer.num_spot_lights = num_spot_lights;
        }

        self.constant_buffer.bind(device_context)
    }
}
