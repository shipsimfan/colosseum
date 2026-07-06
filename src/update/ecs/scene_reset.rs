use crate::update::{ECS, ecs::ArchetypeSet};

impl ECS {
    /// Clear the ECS data to its default state for a new scene
    pub fn scene_reset(&mut self) {
        self.archetypes = ArchetypeSet::new();
    }
}
