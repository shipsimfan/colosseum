use crate::{
    graphics::{Material, Shader},
    math::Color3f,
};

impl Material {
    /// Set the color used by the material
    pub fn set_color(&mut self, color: Color3f) {
        self.buffer.color = color;
    }

    /// Set the strength of the specular highlight
    pub fn set_specular_strength(&mut self, specular_strength: f32) {
        self.buffer.specular_strength = specular_strength;
    }

    /// Set the shader used by this material
    pub fn set_shader(&mut self, shader: Shader) {
        self.shader = shader;
    }
}
