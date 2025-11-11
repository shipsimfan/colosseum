use crate::graphics::{GraphicsContext, Material, Shader};

impl GraphicsContext {
    /// Get the default shader
    pub fn default_shader(&self) -> Shader {
        self.managed_objects.default_shader()
    }

    /// Get the default material
    pub fn default_material(&self) -> Material {
        self.managed_objects.default_material()
    }
}
