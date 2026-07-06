use crate::update::UpdateContext;

impl<'a, Game: crate::Game> UpdateContext<'a, Game> {
    /// Reset the data to its default state for a new scene
    pub(in crate::update) fn scene_reset(&mut self) {
        self.render_data.scene_reset();
        self.ecs.scene_reset();
    }
}
