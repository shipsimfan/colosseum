use crate::graphics::util::{RenderTargetTexture, TextureSampler};
use win32::d3d11::ID3D11DeviceContext;

impl RenderTargetTexture {
    /// Bind this texture to be an active shader resource on `device_context`
    pub fn bind(&mut self, sampler: &mut TextureSampler, device_context: &mut ID3D11DeviceContext) {
        let view = self.shader_view.as_mut() as *mut _;
        device_context.ps_set_shader_resources(self.shader_slot, 1, &view);
        device_context.vs_set_shader_resources(self.shader_slot, 1, &view);

        sampler.bind(self.shader_slot, device_context);
    }
}
