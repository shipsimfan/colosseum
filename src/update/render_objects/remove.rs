use crate::{
    render::{MaterialId, MaterialKind, RenderData, ShaderId, ShaderKind},
    update::UpdateRenderObjects,
};

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
}
