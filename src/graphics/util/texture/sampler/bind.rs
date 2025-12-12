use crate::graphics::util::TextureSampler;
use win32::d3d11::ID3D11DeviceContext;

impl TextureSampler {
    /// Bind this sampler to be the active one on `slot` on `device_context`
    pub(in crate::graphics::util::texture) fn bind(
        &mut self,
        slot: u32,
        device_context: &mut ID3D11DeviceContext,
    ) {
        let sampler = self.sampler.as_mut() as *mut _;
        device_context.vs_set_samplers(slot, 1, &sampler);
        device_context.ps_set_samplers(slot, 1, &sampler);
    }
}
