use crate::graphics::{
    context::{PostProcessing, SwapchainObjects},
    util::RenderTargetTexture,
};
use std::ptr::null_mut;
use win32::d3d11::ID3D11DeviceContext;

impl PostProcessing {
    /// Run the post-processing steps needed, writing the output to `swapchain`
    pub(in crate::graphics::context) fn run(
        &mut self,
        swapchain: &mut SwapchainObjects,
        device_context: &mut ID3D11DeviceContext,
    ) {
        // TODO: Set vertex shader and mesh

        // Provided post-processing steps
        let color_correction_input = self.provided_post_processing(device_context);

        // Color correction
        self.color_correction(color_correction_input, swapchain, device_context);

        // Anti-aliasing
        self.anti_aliasing(swapchain, device_context);

        // Render scale
        self.render_scale(swapchain, device_context);
    }

    /// Run the provided post-processing steps
    fn provided_post_processing<'a>(
        &'a mut self,
        device_context: &mut ID3D11DeviceContext,
    ) -> &'a mut RenderTargetTexture {
        device_context.rs_set_viewports(1, &self.render_scale_objects.viewport);

        let mut input = &mut self.render_scale_objects.hdr_output1;
        let mut output = &mut self.render_scale_objects.hdr_output2;
        for shader in &mut self.provided_post_processing {
            device_context.om_set_render_targets(1, &output.render_view(), null_mut());
            input.bind(device_context);

            // TODO: Set pixel shader and make draw call

            input.unbind(device_context);

            std::mem::swap(&mut input, &mut output);
        }

        input
    }

    /// Run the color correction shader
    fn color_correction(
        &mut self,
        input: &mut RenderTargetTexture,
        swapchain: &mut SwapchainObjects,
        device_context: &mut ID3D11DeviceContext,
    ) {
        // Determine where to render to
        let (color_correction_output, color_correction_viewport) = match (
            &mut self.render_scale_objects.anti_aliasing_input.as_mut(),
            &mut self.render_scale_objects.render_scale_input.as_mut(),
        ) {
            // Anti-aliasing is in use, use the anti-aliasing input texture
            (Some(anti_aliasing_input), _) => (
                anti_aliasing_input.render_view(),
                &self.render_scale_objects.viewport,
            ),

            // No anti-aliasing but render scale is in use, use the render-scale input texture
            (None, Some(render_scale_input)) => (
                render_scale_input.render_view(),
                &self.render_scale_objects.viewport,
            ),

            // No anti-aliasing or render scale, use the swapchain
            (None, None) => swapchain.render_view(),
        };

        // Bind the inputs and outputs
        device_context.rs_set_viewports(1, color_correction_viewport);
        device_context.om_set_render_targets(1, &color_correction_output, null_mut());
        input.bind(device_context);

        // TODO: Set pixel shader and make draw call

        // Unbind the input
        input.unbind(device_context);
    }

    /// Run the anti-aliasing shader if needed
    fn anti_aliasing(
        &mut self,
        swapchain: &mut SwapchainObjects,
        device_context: &mut ID3D11DeviceContext,
    ) {
        // See if we need anti-aliasing
        let anti_aliasing = match self.anti_aliasing {
            Some(anti_aliasing) => anti_aliasing,
            None => return,
        };

        // Get the anti-alias input
        let anti_aliasing_input = self
            .render_scale_objects
            .anti_aliasing_input
            .as_mut()
            .unwrap();

        // Determine where to render to
        let (anti_aliasing_output, anti_aliasing_viewport) =
            match self.render_scale_objects.render_scale_input.as_mut() {
                Some(render_scale_input) => (
                    render_scale_input.render_view(),
                    &self.render_scale_objects.viewport,
                ),
                None => swapchain.render_view(),
            };

        // Bind the inputs and outputs
        device_context.rs_set_viewports(1, anti_aliasing_viewport);
        device_context.om_set_render_targets(1, &anti_aliasing_output, null_mut());
        anti_aliasing_input.bind(device_context);

        // TODO: Set pixel shader and make draw call

        // Unbind the input
        anti_aliasing_input.unbind(device_context);
    }

    /// Scale the output if needed
    fn render_scale(
        &mut self,
        swapchain: &mut SwapchainObjects,
        device_context: &mut ID3D11DeviceContext,
    ) {
        // See if we need render scaling
        let render_scale_input = match &mut self.render_scale_objects.render_scale_input {
            Some(render_scale_input) => render_scale_input,
            None => return,
        };

        // Get the output
        let (render_scale_output, render_scale_viewport) = swapchain.render_view();

        // Bind the inputs and outputs
        device_context.rs_set_viewports(1, render_scale_viewport);
        device_context.om_set_render_targets(1, &render_scale_output, null_mut());
        render_scale_input.bind(device_context);

        // TODO: Set pixel shader and make draw call

        // Unbind the input
        render_scale_input.unbind(device_context);
    }
}
