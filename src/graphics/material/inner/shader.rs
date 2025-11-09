use crate::graphics::{MaterialInner, Shader};
use std::rc::Rc;

impl MaterialInner {
    /// Get the shader this material uses
    pub fn shader(&self) -> &Rc<Shader> {
        &self.shader
    }
}
