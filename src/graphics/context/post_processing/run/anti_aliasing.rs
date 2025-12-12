use crate::graphics::{
    AntiAliasing, CameraPostProcessing,
    context::{PostProcessing, SwapchainObjects},
};
use std::ptr::null_mut;
use win32::d3d11::ID3D11DeviceContext;

impl PostProcessing {
    /// Run the anti-aliasing shader if needed
    pub(in crate::graphics::context::post_processing::run) fn anti_aliasing(
        &mut self,
        camera: &mut CameraPostProcessing,
        swapchain: &mut SwapchainObjects,
        device_context: &mut ID3D11DeviceContext,
    ) {
        let rso = &mut camera.render_scale_objects;

        // See if we need anti-aliasing
        let anti_aliasing = match rso.anti_aliasing() {
            Some(anti_aliasing) => anti_aliasing,
            None => return,
        };

        // Get the anti-alias input
        let anti_aliasing_input = rso.anti_aliasing_input.as_mut().unwrap();

        // Determine where to render to
        let (anti_aliasing_output, anti_aliasing_viewport) = match rso.render_scale_input.as_mut() {
            Some(render_scale_input) => {
                (render_scale_input.render_view(), &rso.render_scale_viewport)
            }
            None => (swapchain.render_view(), &rso.screen_viewport),
        };

        // Bind the inputs and outputs
        device_context.rs_set_viewports(1, anti_aliasing_viewport);
        device_context.om_set_render_targets(1, &anti_aliasing_output, null_mut());
        anti_aliasing_input.bind(&mut self.anti_aliasing_sampler, device_context);

        // Set pixel shader
        match anti_aliasing {
            AntiAliasing::FXAA => self.fxaa_shader.bind(device_context),
        }

        // Make draw call
        device_context.draw_indexed(6, 0, 0);

        // Unbind the input
        anti_aliasing_input.unbind(device_context);
    }
}
