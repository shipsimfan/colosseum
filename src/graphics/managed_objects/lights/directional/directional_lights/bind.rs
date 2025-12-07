use crate::{Result, graphics::DirectionalLights};
use win32::d3d11::{ID3D11Device, ID3D11DeviceContext};

impl DirectionalLights {
    /// Bind the set of [`DirectionalLights`] to `device_context`
    pub(in crate::graphics::managed_objects::lights) fn bind(
        &mut self,
        device: &ID3D11Device,
        device_context: &mut ID3D11DeviceContext,
    ) -> Result<Option<u32>> {
        self.list.bind(device, device_context)
    }
}
