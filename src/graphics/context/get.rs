use crate::{
    graphics::{GraphicsContext, Material, Shader},
    math::Color3f,
};

impl GraphicsContext {
    /// Get the default lit shader
    pub fn default_lit_shader(&self) -> Shader {
        self.managed_objects.default_lit_shader()
    }

    /// Get the default unlit shader
    pub fn default_unlit_shader(&self) -> Shader {
        self.managed_objects.default_unlit_shader()
    }

    /// Get the default lit material
    pub fn default_lit_material(&self) -> Material {
        self.managed_objects.default_lit_material()
    }

    /// Get the default unlit material
    pub fn default_unlit_material(&self) -> Material {
        self.managed_objects.default_unlit_material()
    }

    /// Get the ambient light color
    pub fn ambient_color(&self) -> Color3f {
        self.managed_objects.ambient_color()
    }

    /// Get the intensity of the ambient light
    pub fn ambient_intensity(&self) -> f32 {
        self.managed_objects.ambient_intensity()
    }
}
