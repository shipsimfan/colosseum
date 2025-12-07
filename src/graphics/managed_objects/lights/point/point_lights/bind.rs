use crate::{Result, graphics::PointLights};
use win32::d3d11::{ID3D11Device, ID3D11DeviceContext};

impl PointLights {
    /// Bind the set of [`PointLights`] to `device_context`
    pub(in crate::graphics::managed_objects::lights) fn bind(
        &mut self,
        device: &ID3D11Device,
        device_context: &mut ID3D11DeviceContext,
    ) -> Result<Option<u32>> {
        self.list.bind(device, device_context)
    }
}
