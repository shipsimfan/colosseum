use crate::{
    Result,
    render::{GraphicsDevice, Shader, ShaderCode, ShaderKind},
};
use colosseum_core::new_error;
use std::sync::Arc;

impl Shader {
    /// Create a new [`Shader`] from [`ShaderCode`]
    pub fn new<const N: usize>(
        code: &ShaderCode<N>,
        kind: ShaderKind,
        graphics_device: &GraphicsDevice,
    ) -> Result<Arc<Shader>> {
        let module = graphics_device
            .create_shader_module(code)
            .map_err(|error| new_error!("unable to create a shader - {}", error))?;

        Ok(Arc::new(Shader { module, kind }))
    }
}
