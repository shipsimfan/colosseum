use crate::{
    Result,
    render::{FixedRenderObjects, Pipeline, SDR_FORMAT, Shader, frame_graph::FxaaNode},
};
use alexandria::{
    gpu::{VulkanDevice, VulkanFormat, compile_shader},
    math::Vector2f,
};

compile_shader! {
    const FRAGMENT_SHADER = "fxaa.slang",
    frag_main
}

impl FxaaNode {
    /// Create the persistent objects that are used by this node
    pub(in crate::render::frame_graph::nodes) fn create_fixed_objects(
        fixed_render_objects: &mut FixedRenderObjects,
        _: VulkanFormat,
        device: &VulkanDevice,
    ) -> Result<()> {
        // Create the pipeline
        let shader = Shader::new(&FRAGMENT_SHADER, device)?;
        fixed_render_objects.add_pipeline(
            Pipeline::new_post_process(
                fixed_render_objects.fullscreen_quad(),
                &shader,
                std::mem::size_of::<Vector2f>(),
                None,
                &[SDR_FORMAT],
                &[fixed_render_objects
                    .descriptor_set_layout(FixedRenderObjects::POST_PROCESS_DESCRIPTOR_SET_LAYOUT)],
                device,
            )?,
            FixedRenderObjects::FXAA_PIPELINE,
        );

        Ok(())
    }
}
