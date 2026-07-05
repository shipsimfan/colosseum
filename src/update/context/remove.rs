use crate::{
    render::{Material, Shader},
    update::UpdateContext,
};
use alexandria::Id;

impl<'a, Game: crate::Game> UpdateContext<'a, Game> {
    /// Removes a [`Shader`] from the game
    pub fn remove_shader(&mut self, shader: Id<Shader>) {
        self.shaders.remove(unsafe { shader.cast() });
    }

    /// Removes a [`Material`] from the game
    pub fn remove_material(&mut self, material: Id<Material>) {
        self.materials.remove(material);
        self.render_data.material_change(material);
    }
}
