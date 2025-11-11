use crate::graphics::{Shader, ShaderInner};
use std::ops::Deref;

impl Deref for Shader {
    type Target = ShaderInner;

    fn deref(&self) -> &Self::Target {
        &self.shader
    }
}
