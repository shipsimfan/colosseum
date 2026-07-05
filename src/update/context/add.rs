use crate::{render::Material, update::UpdateContext};
use alexandria::Id;

impl<'a, Game: crate::Game> UpdateContext<'a, Game> {
    /// Add a new renderable object to the render data
    ///
    /// TODO: Replace this with the ECS system
    pub fn add_renderable(&mut self, material: Id<Material>) {
        self.render_data.renderable(material);
    }
}
