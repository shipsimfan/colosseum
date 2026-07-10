use crate::{
    render::{MaterialId, ShaderId},
    update::UpdateContext,
};

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
}
