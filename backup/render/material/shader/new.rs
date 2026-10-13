use crate::{
    Error, Result,
    render::{Shader, ShaderCode},
};
use alexandria::gpu::VulkanDevice;
use std::sync::Arc;

impl Shader {
    /// Create a new [`Shader`] from [`ShaderCode`]
    pub fn new<const N: usize>(code: &ShaderCode<N>, device: &VulkanDevice) -> Result<Arc<Shader>> {
        let module = device
            .create_shader_module(code)
            .map_err(Error::new_inner)?;

        Ok(Arc::new(Shader { module }))
    }
}
