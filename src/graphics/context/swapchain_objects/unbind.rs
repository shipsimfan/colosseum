use crate::graphics::context::SwapchainObjects;
use std::ptr::{null, null_mut};
use win32::d3d11::ID3D11DeviceContext;

impl SwapchainObjects {
    /// Unbind all swapchain objects from the device context
    pub fn unbind(&mut self, device_context: &mut ID3D11DeviceContext) {
        device_context.om_set_render_targets(0, null(), null_mut());
    }
}
