use crate::{
    render::{MaterialId, MaterialKind, Mesh, RenderData, ShaderId, ShaderKind},
    update::UpdateRenderObjects,
};
use alexandria::Id;

impl UpdateRenderObjects {
    /// Removes a shader from the game
    pub fn remove_shader(&mut self, shader: ShaderId) {
        match shader.kind() {
            ShaderKind::Unlit => self.unlit_shaders.remove(shader.id()),
        };
    }

    /// Removes a material from the game
    pub fn remove_material(&mut self, material: MaterialId, render_data: &mut RenderData) {
        match material.kind() {
            MaterialKind::UnlitOpaque => self.unlit_opaque_materials.remove(material.id()),
        };
        render_data.add_render_object_change(material);
    }

    /// Remove a mesh from the game
    pub fn remove_mesh(&mut self, mesh: Id<Mesh>, render_data: &mut RenderData) {
        if let Some((_, memory)) = self.meshes.remove(unsafe { mesh.cast() }) {
            render_data.add_render_object_change((mesh, memory));
        }
    }
}
