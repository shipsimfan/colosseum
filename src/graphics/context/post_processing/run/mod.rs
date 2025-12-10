use crate::graphics::{
    CameraPostProcessing,
    context::{PostProcessing, SwapchainObjects},
};
use win32::d3d11::ID3D11DeviceContext;

mod anti_aliasing;
mod color_correction;
mod provided;
mod render_scale;

impl PostProcessing {
    /// Run the post-processing steps needed, writing the output to `swapchain`
    pub(in crate::graphics::context) fn run(
        &mut self,
        camera: &mut CameraPostProcessing,
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
        let color_correction_input = self.provided_post_processing(camera, device_context);

        // Color correction
        self.color_correction(color_correction_input, camera, swapchain, device_context);

        // Anti-aliasing
        self.anti_aliasing(camera, swapchain, device_context);

        // Render scale
        self.render_scale(camera, swapchain, device_context);
    }
}
