use crate::{
    render::RenderData,
    update::{ECS, SystemId},
};

impl ECS {
    /// Execute an ad hoc system on the archetypes in the ECS system
    pub fn execute_system(&mut self, system: SystemId) {
        self.archetypes.execute_system(system);
    }

    /// Execute all pre-update systems on the archetypes in the ECS system
    pub(in crate::update) fn execute_pre_update_systems(&mut self) {
        self.archetypes.execute_pre_update_systems();
    }

    /// Execute all post-update systems on the archetypes in the ECS system
    pub(in crate::update) fn execute_post_update_systems(&mut self) {
        self.archetypes.execute_post_update_systems();
    }

    /// Execute all rendering systems on the archetypes in the ECS system
    pub(in crate::update) fn execute_rendering_systems(&mut self, render_data: &mut RenderData) {
        self.archetypes.execute_rendering_systems(render_data);
    }
}
