use crate::graphics::context::SwapchainObjects;
use win32::d3d11::{D3D11_CLEAR_FLAG, ID3D11DeviceContext};

impl SwapchainObjects {
    /// Clear the swapchain's back buffer
    pub fn clear(&mut self, device_context: &mut ID3D11DeviceContext, color: [f32; 4]) {
        device_context.clear_render_target_view(self.render_target_view.as_mut(), color);
        device_context.clear_depth_stencil_view(
            self.depth_stencil_view.as_mut(),
            D3D11_CLEAR_FLAG::Depth as _,
            1.0,
            0,
        );
    }
}
