use crate::{
    Result,
    render::{
        MaterialId, MaterialKind, Mesh, MeshTransfer, ShaderCode, ShaderId, ShaderKind, Vertex,
    },
    update::UpdateContext,
};
use alexandria::Id;

impl<'a, Game: crate::Game> UpdateContext<'a, Game> {
    /// Create a new shader
    pub fn create_shader<const N: usize>(
        &mut self,
        kind: ShaderKind,
        code: &ShaderCode<N>,
    ) -> Result<ShaderId> {
        self.render_objects.create_shader(kind, code)
    }

    /// Create a new material
    pub fn create_material(&mut self, kind: MaterialKind, shader: ShaderId) -> Result<MaterialId> {
        self.render_objects
            .create_material(kind, shader, self.render_data)
    }

    /// Create a new mesh
    ///
    /// The mesh cannot be used in rendering until the [`MeshTransfer`] has completed
    pub fn create_mesh(
        &mut self,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
    ) -> Result<(Id<Mesh>, MeshTransfer)> {
        self.render_objects.create_mesh(vertices, indices)
    }
}
