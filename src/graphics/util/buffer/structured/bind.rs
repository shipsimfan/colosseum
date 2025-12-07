use crate::graphics::util::StructuredBuffer;
use win32::d3d11::ID3D11DeviceContext;

impl<T: Sized + Copy> StructuredBuffer<T> {
    /// Bind this structured buffer to be an active one on the `device_context`
    pub fn bind(&mut self, device_context: &mut ID3D11DeviceContext) {
        let view = self.view.as_mut() as *mut _;
        device_context.vs_set_shader_resources(self.slot, 1, &view);
        device_context.ps_set_shader_resources(self.slot, 1, &view);
    }
}
