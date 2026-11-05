use crate::{
    Result,
    render::{
        GraphicsDevice, ShaderCode, UnlitMaterial, UnlitShader,
        public_data::materials::UnlitMaterials,
    },
};
use alexandria::Id;
use std::sync::Arc;

impl UnlitMaterials {
    /// Insert a new unlit material
    pub fn insert(&mut self, material: UnlitMaterial) -> Id<UnlitMaterial> {
        self.materials.insert(material)
    }

    /// Create a new unlit shader
    pub fn create_shader<const N: usize>(
        &mut self,
        code: &ShaderCode<N>,
        device: &GraphicsDevice,
    ) -> Result<Arc<UnlitShader>> {
        UnlitShader::new(code, &self.pipeline_layout, device)
    }
}
