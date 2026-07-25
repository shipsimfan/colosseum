use crate::{
    Result,
    render::{FixedRenderObjects, Shader},
};
use alexandria::gpu::{VulkanDevice, compile_shader};

compile_shader! {
    /// The vertex shader code for the fullscreen quad shader
    const FULLSCREEN_QUAD_SHADER = "fullscreen-quad.slang",
    main
}

impl FixedRenderObjects {
    /// Create a new set of [`FixedRenderObjects`]
    pub fn new(device: &VulkanDevice) -> Result<FixedRenderObjects> {
        let fullscreen_quad = Shader::new(&FULLSCREEN_QUAD_SHADER, device)?;

        Ok(FixedRenderObjects {
            fullscreen_quad,
            pipelines: Vec::new(),
        })
    }
}
