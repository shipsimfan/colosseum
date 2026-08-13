use crate::{render::Skybox, update::UpdateContext};

impl<'a, Game: crate::Game> UpdateContext<'a, Game> {
    /// Reset the data to its default state for a new scene
    pub(in crate::update) fn scene_reset(&mut self) {
        *self.skybox = Skybox::default();

        self.ecs.scene_reset();
        *self.active_camera = None;
    }
}
