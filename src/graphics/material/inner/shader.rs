use crate::graphics::{MaterialInner, Shader};

impl MaterialInner {
    /// Get the shader this material uses
    pub fn shader(&self) -> &Shader {
        &self.shader
    }
}
