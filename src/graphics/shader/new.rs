use crate::{
    Result,
    graphics::{Shader, ShaderInner, ShaderSource},
};
use std::{num::NonZeroU32, rc::Rc};
use win32::d3d11::ID3D11Device;

impl Shader {
    /// Create a new [`Shader`]
    pub(in crate::graphics) fn new(
        id: NonZeroU32,
        compiled_shader: &ShaderSource,
        device: &ID3D11Device,
    ) -> Result<Self> {
        let shader = Rc::new(ShaderInner::new(id, compiled_shader, device)?);

        Ok(Shader { shader })
    }
}
