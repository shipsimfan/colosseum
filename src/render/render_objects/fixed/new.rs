use crate::{
    Result,
    render::{FixedRenderObjects, FrameGraphNode, Shader},
};
use alexandria::gpu::{VulkanDevice, VulkanFormat, compile_shader};
use std::sync::Arc;

compile_shader! {
    /// The vertex shader code for the fullscreen quad shader
    const FULLSCREEN_QUAD_SHADER = "fullscreen-quad.slang",
    vert_main
}

impl FixedRenderObjects {
    /// Create a new set of [`FixedRenderObjects`]
    pub fn new(
        swapchain_format: VulkanFormat,
        device: &VulkanDevice,
    ) -> Result<Arc<FixedRenderObjects>> {
        let mut fixed_render_objects = FixedRenderObjects {
            pipeline_layouts: Vec::new(),
            pipelines: Vec::new(),
            samplers: Vec::new(),

            descriptor_set_layouts: Vec::new(),
            max_descriptor_sets: 0,
            descriptor_pool_sizes: Vec::new(),

            fullscreen_quad: Shader::new(&FULLSCREEN_QUAD_SHADER, device)?,
        };

        FrameGraphNode::create_objects(&mut fixed_render_objects, swapchain_format, device)?;

        Ok(Arc::new(fixed_render_objects))
    }
}
