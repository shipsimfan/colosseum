use crate::graphics::{
    CameraPostProcessing,
    context::{PostProcessing, SwapchainObjects},
};
use std::ptr::null_mut;
use win32::d3d11::ID3D11DeviceContext;

impl PostProcessing {
    /// Run the color correction shader
    pub(in crate::graphics::context::post_processing::run) fn color_correction(
        &mut self,
        input: bool,
        camera: &mut CameraPostProcessing,
        swapchain: &mut SwapchainObjects,
        device_context: &mut ID3D11DeviceContext,
    ) {
        let rso = &mut camera.render_scale_objects;

        // Determine the input
        let input = if input {
            &mut rso.hdr_output2
        } else {
            &mut rso.hdr_output1
        };

        // Determine where to render to
        let (color_correction_output, color_correction_viewport) = match (
            &mut rso.anti_aliasing_input.as_mut(),
            &mut rso.render_scale_input.as_mut(),
        ) {
            // Anti-aliasing is in use, use the anti-aliasing input texture
            (Some(anti_aliasing_input), _) => (
                anti_aliasing_input.render_view(),
                &rso.render_scale_viewport,
            ),

            // No anti-aliasing but render scale is in use, use the render-scale input texture
            (None, Some(render_scale_input)) => {
                (render_scale_input.render_view(), &rso.render_scale_viewport)
            }

            // No anti-aliasing or render scale, use the swapchain
            (None, None) => (swapchain.render_view(), &rso.screen_viewport),
        };

        // Bind the inputs and outputs
        device_context.rs_set_viewports(1, color_correction_viewport);
        device_context.om_set_render_targets(1, &color_correction_output, null_mut());
        input.bind(&mut self.sampler, device_context);

        // Set pixel shader
        self.color_correction_shader.bind(device_context);

        // Make draw call
        device_context.draw_indexed(6, 0, 0);

        // Unbind the input
        input.unbind(device_context);
    }
}
