use crate::{
    Result,
    render::{
        GraphicsDevice, LitForwardMaterial, LitForwardShader, ShaderCode,
        public_data::materials::LitForwardMaterials,
    },
};
use alexandria::Id;
use std::sync::Arc;

impl LitForwardMaterials {
    /// Insert a new lit forward material
    pub fn insert(&mut self, material: LitForwardMaterial) -> Id<LitForwardMaterial> {
        self.materials.insert(material)
    }

    /// Create a new lit forward shader
    pub fn create_shader<const N: usize>(
        &mut self,
        code: &ShaderCode<N>,
        device: &GraphicsDevice,
    ) -> Result<Arc<LitForwardShader>> {
        LitForwardShader::new(code, &self.pipeline_layout, device)
    }
}
