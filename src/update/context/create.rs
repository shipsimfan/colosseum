use crate::{
    Result,
    render::{Material, Shader, ShaderCode},
    update::UpdateContext,
};
use alexandria::Id;
use std::sync::Arc;

impl<'a, Game: crate::Game> UpdateContext<'a, Game> {
    /// Create a new [`Shader`]
    pub fn create_shader<const N: usize>(&mut self, code: &ShaderCode<N>) -> Result<Id<Shader>> {
        let shader = Shader::new(code, self.device)?;
        let id = self.shaders.insert(Arc::new(shader));
        Ok(unsafe { id.cast() })
    }

    /// Create a new [`Material`]
    pub fn create_material(&mut self, shader: Id<Shader>) -> Result<Id<Material>> {
        let (material, render_material) = Material::new(
            &self.shaders[unsafe { shader.cast() }],
            self.pipeline_layout,
            self.swapchain_format,
            self.device,
        )?;
        self.render_data.material_change(render_material);
        Ok(self.materials.insert(material))
    }
}
