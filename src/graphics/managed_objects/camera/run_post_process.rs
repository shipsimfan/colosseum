use crate::graphics::{Camera, context::SwapchainObjects};
use win32::d3d11::ID3D11DeviceContext;

impl Camera {
    pub(in crate::graphics) fn run_post_process(
        &mut self,
        swapchain: &mut SwapchainObjects,
        device_context: &mut ID3D11DeviceContext,
    ) {
        self.post_processing.run(swapchain, device_context)
    }
}
