use crate::graphics::{
    CameraPostProcessing,
    context::{PostProcessing, SwapchainObjects},
};
use std::ptr::null_mut;
use win32::d3d11::ID3D11DeviceContext;

mod anti_aliasing;
mod color_correction;
mod provided;

impl PostProcessing {
    /// Run the post-processing steps needed, writing the output to `swapchain`
    pub(in crate::graphics::context) fn run(
        &mut self,
        camera_post_processing: &mut CameraPostProcessing,
        swapchain: &mut SwapchainObjects,
        device_context: &mut ID3D11DeviceContext,
    ) {
        // Set sampler, vertex shader, and mesh
        device_context.om_set_depth_stencil_state(self.depth_stencil_state.as_mut(), 0);
        let sampler = self.sampler_state.as_mut() as *mut _;
        device_context.ps_set_samplers(0, 1, &sampler);
        self.vertex_buffer.bind(device_context);
        self.index_buffer.bind(device_context);
        self.vertex_shader.bind(device_context);

        // Provided post-processing steps
        let color_correction_input = self.provided_post_processing(device_context);

        // Color correction
        self.color_correction(color_correction_input, swapchain, device_context);

        // Anti-aliasing
        self.anti_aliasing(swapchain, device_context);

        // Render scale
        self.render_scale(swapchain, device_context);
    }

    /// Run the provided post-processing steps, returning `true` if the final output was HDR2 or
    /// `false` if the final output was HDR1
    fn provided_post_processing(
        &mut self,
        camera_post_processing: &mut CameraPostProcessing,
        device_context: &mut ID3D11DeviceContext,
    ) -> bool {
        let mut input = &mut self.render_scale_objects.hdr_output1;
        let mut output = &mut self.render_scale_objects.hdr_output2;
        let mut final_output = false;
        for shader in &mut self.provided_post_processing {
            // Bind the inputs and outputs
            device_context.om_set_render_targets(1, &output.render_view(), null_mut());
            input.bind(device_context);

            // Bind pixel shader
            shader.bind(device_context);

            // Make draw call
            device_context.draw_indexed(6, 0, 0);

            input.unbind(device_context);

            std::mem::swap(&mut input, &mut output);
            final_output = !final_output;
        }

        final_output
    }

    /// Run the color correction shader
    fn color_correction(
        &mut self,
        input: bool,
        camera_post_processing: &mut CameraPostProcessing,
        swapchain: &mut SwapchainObjects,
        device_context: &mut ID3D11DeviceContext,
    ) {
        // Determine the input
        let input = if input {
            &mut self.render_scale_objects.hdr_output2
        } else {
            &mut self.render_scale_objects.hdr_output1
        };

        // Determine where to render to
        let (color_correction_output, color_correction_viewport) = match (
            &mut self.render_scale_objects.anti_aliasing_input.as_mut(),
            &mut self.render_scale_objects.render_scale_input.as_mut(),
        ) {
            // Anti-aliasing is in use, use the anti-aliasing input texture
            (Some(anti_aliasing_input), _) => (
                anti_aliasing_input.render_view(),
                &self.render_scale_objects.render_scale_viewport,
            ),

            // No anti-aliasing but render scale is in use, use the render-scale input texture
            (None, Some(render_scale_input)) => (
                render_scale_input.render_view(),
                &self.render_scale_objects.render_scale_viewport,
            ),

            // No anti-aliasing or render scale, use the swapchain
            (None, None) => (
                swapchain.render_view(),
                &self.render_scale_objects.screen_viewport,
            ),
        };

        // Bind the inputs and outputs
        device_context.rs_set_viewports(1, color_correction_viewport);
        device_context.om_set_render_targets(1, &color_correction_output, null_mut());
        input.bind(device_context);

        // Set pixel shader
        self.color_correction_shader.bind(device_context);

        // Make draw call
        device_context.draw_indexed(6, 0, 0);

        // Unbind the input
        input.unbind(device_context);
    }

    /// Run the anti-aliasing shader if needed
    fn anti_aliasing(
        &mut self,
        camera_post_processing: &mut CameraPostProcessing,
        swapchain: &mut SwapchainObjects,
        device_context: &mut ID3D11DeviceContext,
    ) {
        // See if we need anti-aliasing
        let anti_aliasing = match self.render_scale_objects.anti_aliasing {
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
                    &self.render_scale_objects.render_scale_viewport,
                ),
                None => (
                    swapchain.render_view(),
                    &self.render_scale_objects.screen_viewport,
                ),
            };

        // Bind the inputs and outputs
        device_context.rs_set_viewports(1, anti_aliasing_viewport);
        device_context.om_set_render_targets(1, &anti_aliasing_output, null_mut());
        anti_aliasing_input.bind(device_context);

        // TODO: Set pixel shader

        // Make draw call
        device_context.draw_indexed(6, 0, 0);

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
        let render_scale_output = swapchain.render_view();
        let render_scale_viewport = &self.render_scale_objects.screen_viewport;

        // Bind the inputs and outputs
        device_context.rs_set_viewports(1, render_scale_viewport);
        device_context.om_set_render_targets(1, &render_scale_output, null_mut());
        render_scale_input.bind(device_context);

        // Set pixel shader
        self.render_scale_shader.bind(device_context);

        // Make draw call
        device_context.draw_indexed(6, 0, 0);

        // Unbind the input
        render_scale_input.unbind(device_context);
    }
}
