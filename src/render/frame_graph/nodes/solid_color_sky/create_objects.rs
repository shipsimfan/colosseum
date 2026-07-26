use crate::{
    Result,
    render::{Pipeline, Shader, frame_graph::SolidColorSkyNode},
};
use alexandria::{
    gpu::{VulkanDevice, VulkanFormat, compile_shader},
    math::{Color4f, Linear},
};
use std::sync::Arc;

compile_shader! {
    const FRAGMENT_SHADER = "solid-color-sky.slang",
    frag_main
}

impl SolidColorSkyNode {
    /// Create the persistent objects that are used by this node
    pub(in crate::render) fn create_objects(
        pipelines: &mut Vec<Pipeline>,
        fullscreen_quad: &Arc<Shader>,
        swapchain_format: VulkanFormat,
        device: &VulkanDevice,
    ) -> Result<()> {
        assert_eq!(pipelines.len(), 0);

        let shader = Shader::new(&FRAGMENT_SHADER, device)?;

        let pipeline = Pipeline::new_post_process(
            fullscreen_quad,
            &shader,
            std::mem::size_of::<Color4f<Linear>>(),
            swapchain_format,
            device,
        )?;

        pipelines.push(pipeline);

        Ok(())
    }
}
