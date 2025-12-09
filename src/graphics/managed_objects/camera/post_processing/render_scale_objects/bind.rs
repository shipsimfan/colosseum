use crate::graphics::managed_objects::camera::post_processing::RenderScaleObjects;
use win32::d3d11::ID3D11DeviceContext;

impl RenderScaleObjects {
    /// Bind the first HDR output texture for rendering
    pub fn bind_main_color_output(&mut self, device_context: &mut ID3D11DeviceContext) {
        device_context.rs_set_viewports(1, &self.render_scale_viewport);

        let render_view = self.hdr_output1.render_view();
        device_context.om_set_render_targets(1, &render_view, self.depth_buffer.depth_view());
    }
}
