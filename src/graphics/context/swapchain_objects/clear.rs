use crate::graphics::context::SwapchainObjects;
use win32::d3d11::ID3D11DeviceContext;

impl SwapchainObjects {
    /// Clear the swapchain's back buffer
    pub fn clear(&mut self, color: [f32; 4], device_context: &mut ID3D11DeviceContext) {
        self.back_buffer.clear(color, device_context);
        self.depth_buffer.clear(1.0, device_context);
    }
}
