use std::ptr::null_mut;

use crate::graphics::context::{PostProcessing, SwapchainObjects};
use win32::d3d11::ID3D11DeviceContext;

impl PostProcessing {
    /// Run the post-processing steps needed, writing the output to `swapchain`
    pub fn run(
        &mut self,
        swapchain: &mut SwapchainObjects,
        device_context: &mut ID3D11DeviceContext,
    ) {
        // TODO: Set vertex shader and mesh

        // Provided post-processing steps
        let mut input = &mut self.hdr_output1;
        let mut output = &mut self.hdr_output2;

        // TODO: handle provided post-processing stages

        // Color correction
        let color_correction_input = input;
        let (color_correction_output, color_correction_viewport) =
            match (&mut self.anti_aliasing, &mut self.render_scale_input) {
                (Some((_, anti_aliasing_input)), _) => {
                    (anti_aliasing_input.render_view(), &self.viewport)
                }
                (None, Some(render_scale_input)) => {
                    (render_scale_input.render_view(), &self.viewport)
                }
                (None, None) => swapchain.render_view(),
            };

        device_context.rs_set_viewports(1, color_correction_viewport);
        device_context.om_set_render_targets(1, &color_correction_output, null_mut());
        color_correction_input.bind(device_context);

        // TODO: Set pixel shader and make draw call

        // Anti-aliasing
        if let Some((anti_aliasing, anti_aliasing_input)) = &mut self.anti_aliasing {
            let (anti_aliasing_output, anti_aliasing_viewport) = match &mut self.render_scale_input
            {
                Some(render_scale_input) => (render_scale_input.render_view(), &self.viewport),
                None => swapchain.render_view(),
            };

            device_context.rs_set_viewports(1, anti_aliasing_viewport);
            device_context.om_set_render_targets(1, &anti_aliasing_output, null_mut());
            anti_aliasing_input.bind(device_context);

            // TODO: Set pixel shader and make draw call
        }

        // Render scale
        if let Some(render_scale_input) = &mut self.render_scale_input {
            let (render_scale_output, render_scale_viewport) = swapchain.render_view();

            device_context.rs_set_viewports(1, render_scale_viewport);
            device_context.om_set_render_targets(1, &render_scale_output, null_mut());
            render_scale_input.bind(device_context);

            // TODO: Set pixel shader and make draw call
        }
    }
}
