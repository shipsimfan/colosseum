use crate::update::UpdateContext;

impl<'a, Game: crate::Game> UpdateContext<'a, Game> {
    /// Execute all rendering systems on the archetypes in the ECS system
    pub(in crate::update) fn execute_rendering_systems(&mut self) {
        self.render_data.set_skybox(self.skybox.clone());

        self.ecs.execute_rendering_systems(self.render_data);
    }
}
