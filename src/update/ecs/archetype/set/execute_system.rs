use crate::{
    render::RenderData,
    update::{PhysicsData, SystemId, SystemPhase, ecs::ArchetypeSet},
};

impl ArchetypeSet {
    /// Execute an ad hoc system on the archetypes in the ECS system
    pub fn execute_system(&mut self, system: SystemId) {
        match system.phase() {
            SystemPhase::PreUpdate => &mut self.pre_update_systems[system.id()],
            SystemPhase::AdHoc => &mut self.ad_hoc_systems[system.id()],
            SystemPhase::PostUpdate => &mut self.post_update_systems[system.id()],
        }
        .execute(&mut self.archetypes, &mut ())
    }

    /// Execute all physics systems on the archetypes in the ECS system
    pub fn execute_physics_systems(&mut self, physics_data: &mut PhysicsData) {
        for system in &mut self.physics_systems {
            system.execute(&mut self.archetypes, physics_data);
        }
    }

    /// Execute all pre-update systems on the archetypes in the ECS system
    pub fn execute_pre_update_systems(&mut self) {
        for system in &mut self.pre_update_systems {
            system.execute(&mut self.archetypes, &mut ());
        }
    }

    /// Execute all post-update systems on the archetypes in the ECS system
    pub fn execute_post_update_systems(&mut self) {
        for system in &mut self.post_update_systems {
            system.execute(&mut self.archetypes, &mut ());
        }
    }

    /// Execute all rendering systems on the archetypes in the ECS system
    pub fn execute_rendering_systems(&mut self, render_data: &mut RenderData) {
        for system in &mut self.rendering_systems {
            system.execute(&mut self.archetypes, render_data);
        }
    }
}
