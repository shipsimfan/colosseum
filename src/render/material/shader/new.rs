use crate::{
    Error, Result,
    render::{Shader, ShaderCode},
};
use alexandria::gpu::VulkanDevice;

impl Shader {
    /// Create a new [`Shader`] from [`ShaderCode`]
    pub(crate) fn new<const N: usize>(
        code: &ShaderCode<N>,
        device: &VulkanDevice,
    ) -> Result<Shader> {
        let module = device
            .create_shader_module(code)
            .map_err(Error::new_inner)?;

        Ok(Shader { module })
    }
}
