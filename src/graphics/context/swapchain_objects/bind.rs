use crate::graphics::context::SwapchainObjects;
use win32::d3d11::ID3D11DeviceContext;

impl SwapchainObjects {
    pub fn bind(&mut self, device_context: &mut ID3D11DeviceContext) {
        let render_target_view = self.render_target_view.as_mut() as *mut _;
        device_context.rs_set_viewports(1, &self.viewport);
        device_context.om_set_render_targets(
            1,
            &render_target_view,
            self.depth_stencil_view.as_mut(),
        );
    }
}
