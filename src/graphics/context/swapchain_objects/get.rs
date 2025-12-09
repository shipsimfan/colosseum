use crate::graphics::context::SwapchainObjects;
use win32::d3d11::ID3D11RenderTargetView;

impl SwapchainObjects {
    /// Get the swapchain's depth buffer
    pub fn render_view(&mut self) -> *mut ID3D11RenderTargetView {
        self.back_buffer.render_view()
    }
}
