use crate::{
    Result,
    graphics::{MaterialHandle, Mesh, MeshRenderer, MeshRendererHandle, MeshRenderers},
};

impl MeshRenderers {
    /// Create a new [`MeshRenderer`]
    pub fn create(
        &mut self,
        material: MaterialHandle,
        mesh: Mesh,
        max_instances: usize,
    ) -> Result<MeshRendererHandle> {
        Ok(self.arena.insert(MeshRenderer::new(
            material,
            mesh,
            max_instances,
            &self.device,
        )?))
    }
}
