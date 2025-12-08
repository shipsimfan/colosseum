use crate::graphics::util::RenderTargetTexture;
use std::ptr::null_mut;
use win32::d3d11::ID3D11DeviceContext;

impl RenderTargetTexture {
    /// Unbind this texture from an active shader resource on `device_context`
    pub fn unbind(&mut self, device_context: &mut ID3D11DeviceContext) {
        let view = null_mut();
        device_context.ps_set_shader_resources(self.shader_slot, 1, &view);
        device_context.vs_set_shader_resources(self.shader_slot, 1, &view);
    }
}
