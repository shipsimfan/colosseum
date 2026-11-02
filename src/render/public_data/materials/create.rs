use crate::{
    Result,
    render::{Materials, ShaderCode, UnlitMaterial, UnlitShader},
};
use alexandria::{
    Id,
    math::{Color4f, Srgb},
};
use std::sync::Arc;

impl Materials {
    /// Create a new unlit material
    pub fn create_unlit_material<C: Into<Color4f<Srgb>>>(
        &mut self,
        color: C,
        shader: Arc<UnlitShader>,
    ) -> Id<UnlitMaterial> {
        self.insert_unlit_material(UnlitMaterial::new(color, shader))
    }

    /// Insert a new unlit material
    pub fn insert_unlit_material(&mut self, material: UnlitMaterial) -> Id<UnlitMaterial> {
        self.unlit_materials.insert(material)
    }

    /// Create a new unlit shader
    pub fn create_unlit_shader<const N: usize>(
        &mut self,
        code: &ShaderCode<N>,
    ) -> Result<Arc<UnlitShader>> {
        UnlitShader::new(code, &self.device)
    }
}
