use crate::{
    render::{MaterialId, Mesh, ShaderId},
    update::UpdateContext,
};
use alexandria::Id;

impl<'a, Game: crate::Game> UpdateContext<'a, Game> {
    /// Removes a shader from the game
    pub fn remove_shader(&mut self, shader: ShaderId) {
        self.render_objects.remove_shader(shader);
    }

    /// Removes a material from the game
    pub fn remove_material(&mut self, material: MaterialId) {
        self.render_objects
            .remove_material(material, self.render_data);
    }

    /// Remove a mesh from the game
    pub fn remove_mesh(&mut self, mesh: Id<Mesh>) {
        self.render_objects.remove_mesh(mesh, self.render_data);
    }
}
