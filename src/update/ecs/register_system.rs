use crate::update::{
    ECS, SystemId, SystemPhase,
    ecs::{Archetype, System},
};
use std::any::TypeId;

impl ECS {
    /// Register a new system with the ECS
    pub fn register_system(
        &mut self,
        phase: SystemPhase,
        system: (&[TypeId], Box<dyn Fn(&mut [Archetype], &[usize])>),
    ) -> SystemId {
        self.archetypes
            .register_system(phase, System::new(system.0, system.1))
    }
}
