use crate::{
    Result,
    graphics::{Shader, ShaderInner, ShaderSource},
};
use std::{num::NonZeroU32, rc::Rc};
use win32::d3d11::ID3D11Device;

impl Shader {
    /// Create a new unlit [`Shader`]
    pub(in crate::graphics) fn new_unlit(
        id: NonZeroU32,
        compiled_shader: &ShaderSource,
        device: &ID3D11Device,
    ) -> Result<Self> {
        let shader = Rc::new(ShaderInner::new_unlit(id, compiled_shader, device)?);

        Ok(Shader { shader })
    }

    /// Create a new lit [`Shader`]
    pub(in crate::graphics) fn new_lit(
        id: NonZeroU32,
        compiled_shader: &ShaderSource,
        device: &ID3D11Device,
    ) -> Result<Self> {
        let shader = Rc::new(ShaderInner::new_lit(id, compiled_shader, device)?);

        Ok(Shader { shader })
    }
}
