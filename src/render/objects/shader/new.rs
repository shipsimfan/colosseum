use crate::{
    Result,
    render::{GraphicsDevice, Shader, ShaderCode},
};
use colosseum_core::new_error;
#[cfg(debug_assertions)]
use std::ffi::CString;

impl Shader {
    /// Create a new [`Shader`] from [`ShaderCode`]
    pub fn new<const N: usize>(code: &ShaderCode<N>, device: &GraphicsDevice) -> Result<Shader> {
        #[cfg_attr(not(debug_assertions), allow(unused_mut))]
        let mut module = device.create_shader_module(code).map_err(|error| {
            new_error!(
                "unable to create \"{}\" shader module - {}",
                code.file_name(),
                error
            )
        })?;

        #[cfg(debug_assertions)]
        device
            .set_object_name(
                &mut module,
                &CString::new(format!("\"{}\" Shader Module", code.file_name())).unwrap(),
            )
            .map_err(|error| {
                new_error!(
                    "unable to name \"{}\" shader module - {}",
                    code.file_name(),
                    error
                )
            })?;

        Ok(Shader { module })
    }
}
