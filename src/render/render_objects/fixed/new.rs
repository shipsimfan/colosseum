use crate::{
    Result,
    render::{FixedRenderObjects, FrameGraphNode, Shader},
};
use alexandria::gpu::{VulkanDevice, VulkanFormat, compile_shader};

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
    ) -> Result<FixedRenderObjects> {
        let fullscreen_quad = Shader::new(&FULLSCREEN_QUAD_SHADER, device)?;

        let mut pipelines = Vec::new();
        FrameGraphNode::create_objects(&mut pipelines, &fullscreen_quad, swapchain_format, device)?;

        Ok(FixedRenderObjects { pipelines })
    }
}
