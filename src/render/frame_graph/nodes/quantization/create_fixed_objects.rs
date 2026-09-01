use crate::{
    Result,
    render::{
        FixedRenderObjects, Pipeline, Shader,
        frame_graph::{QuantizationNode, nodes::quantization::PushConstants},
    },
};
use alexandria::gpu::{VulkanDevice, VulkanFormat, compile_shader};

compile_shader! {
    const FRAGMENT_SHADER = "quantization.slang",
    frag_main
}

impl QuantizationNode {
    /// Create the persistent objects that are used by this node
    pub(in crate::render::frame_graph::nodes) fn create_fixed_objects(
        fixed_render_objects: &mut FixedRenderObjects,
        swapchain_format: VulkanFormat,
        device: &VulkanDevice,
    ) -> Result<()> {
        // Create the quantization pipeline
        let shader = Shader::new(&FRAGMENT_SHADER, device)?;
        fixed_render_objects.add_pipeline(
            Pipeline::new_post_process(
                fixed_render_objects.fullscreen_quad(),
                &shader,
                std::mem::size_of::<PushConstants>(),
                None,
                &[swapchain_format],
                &[fixed_render_objects
                    .descriptor_set_layout(FixedRenderObjects::POST_PROCESS_DESCRIPTOR_SET_LAYOUT)],
                device,
            )?,
            FixedRenderObjects::QUANTIZATION_PIPELINE,
        );

        Ok(())
    }
}
