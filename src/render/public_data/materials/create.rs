use crate::{
    Result,
    render::{
        LitForwardMaterial, LitForwardShader, Materials, ShaderCode, UnlitMaterial, UnlitShader,
    },
};
use alexandria::{
    Id,
    math::{Color3f, Srgb},
};
use std::sync::Arc;

impl Materials {
    /// Create a new unlit material
    pub fn create_unlit<C: Into<Color3f<Srgb>>>(
        &mut self,
        color: C,
        shader: Option<Arc<UnlitShader>>,
    ) -> Id<UnlitMaterial> {
        self.insert_unlit(UnlitMaterial::new(
            color,
            shader.unwrap_or_else(|| self.default_unlit_shader().clone()),
        ))
    }

    /// Insert a new unlit material
    pub fn insert_unlit(&mut self, material: UnlitMaterial) -> Id<UnlitMaterial> {
        self.unlit.insert(material)
    }

    /// Create a new unlit shader
    pub fn create_unlit_shader<const N: usize>(
        &mut self,
        code: &ShaderCode<N>,
    ) -> Result<Arc<UnlitShader>> {
        self.unlit.create_shader(code, &self.device)
    }

    /// Create a new lit forward material
    pub fn create_lit_forward<C: Into<Color3f<Srgb>>>(
        &mut self,
        color: C,
        specular_strength: f32,
        shininess: f32,
        shader: Option<Arc<LitForwardShader>>,
    ) -> Id<LitForwardMaterial> {
        self.insert_lit_forward(LitForwardMaterial::new(
            color,
            specular_strength,
            shininess,
            shader.unwrap_or_else(|| self.default_lit_forward_shader().clone()),
        ))
    }

    /// Insert a new lit forward material
    pub fn insert_lit_forward(&mut self, material: LitForwardMaterial) -> Id<LitForwardMaterial> {
        self.lit_forward.insert(material)
    }

    /// Create a new lit forward shader
    pub fn create_lit_forward_shader<const N: usize>(
        &mut self,
        code: &ShaderCode<N>,
    ) -> Result<Arc<LitForwardShader>> {
        self.lit_forward.create_shader(code, &self.device)
    }
}
