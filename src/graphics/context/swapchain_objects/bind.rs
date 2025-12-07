use crate::graphics::context::SwapchainObjects;
use win32::d3d11::ID3D11DeviceContext;

impl SwapchainObjects {
    pub fn bind(&mut self, device_context: &mut ID3D11DeviceContext) {
        device_context.rs_set_viewports(1, &self.viewport);

        let render_view = self.back_buffer.render_view();
        let depth_view = self.depth_buffer.depth_view();
        device_context.om_set_render_targets(1, &render_view, depth_view);
    }
}
