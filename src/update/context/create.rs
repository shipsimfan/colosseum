use crate::{
    Result,
    render::{MaterialId, MaterialKind, ShaderCode, ShaderId, ShaderKind},
    update::UpdateContext,
};

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
}
