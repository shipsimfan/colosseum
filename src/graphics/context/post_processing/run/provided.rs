use crate::graphics::{CameraPostProcessing, context::PostProcessing};
use std::ptr::null_mut;
use win32::d3d11::ID3D11DeviceContext;

impl PostProcessing {
    /// Run the provided post-processing steps, returning `true` if the final output was HDR2 or
    /// `false` if the final output was HDR1
    pub(in crate::graphics::context::post_processing::run) fn provided_post_processing(
        &mut self,
        camera: &mut CameraPostProcessing,
        device_context: &mut ID3D11DeviceContext,
    ) -> bool {
        let rso = &mut camera.render_scale_objects;

        let mut input = &mut rso.hdr_output1;
        let mut output = &mut rso.hdr_output2;
        let mut final_output = false;
        for shader in &mut camera.provided_post_processing {
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
}
