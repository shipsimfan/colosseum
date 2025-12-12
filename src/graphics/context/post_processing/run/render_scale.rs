use crate::graphics::{
    CameraPostProcessing,
    context::{PostProcessing, SwapchainObjects},
};
use std::ptr::null_mut;
use win32::d3d11::ID3D11DeviceContext;

impl PostProcessing {
    /// Scale the output if needed
    pub(in crate::graphics::context::post_processing::run) fn render_scale(
        &mut self,
        camera: &mut CameraPostProcessing,
        swapchain: &mut SwapchainObjects,
        device_context: &mut ID3D11DeviceContext,
    ) {
        let rso = &mut camera.render_scale_objects;

        // See if we need render scaling
        let render_scale_input = match &mut rso.render_scale_input {
            Some(render_scale_input) => render_scale_input,
            None => return,
        };

        // Get the output
        let render_scale_output = swapchain.render_view();
        let render_scale_viewport = &rso.screen_viewport;

        // Bind the inputs and outputs
        device_context.rs_set_viewports(1, render_scale_viewport);
        device_context.om_set_render_targets(1, &render_scale_output, null_mut());
        render_scale_input.bind(
            if self.render_scale_point {
                &mut self.point_sampler
            } else {
                &mut self.linear_sampler
            },
            device_context,
        );

        // Set pixel shader
        self.render_scale_shader.bind(device_context);

        // Make draw call
        device_context.draw_indexed(6, 0, 0);

        // Unbind the input
        render_scale_input.unbind(device_context);
    }
}
