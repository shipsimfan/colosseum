use crate::{System, Systems, systems};
use std::any::TypeId;

impl Systems {
    /// Register a new phase of systems
    pub fn register_phase<Phase: 'static>(&mut self) {
        let r#type = TypeId::of::<Phase>();
        if self.get_phase(r#type).is_none() {
            self.phases.push((r#type, systems::Phase::new()));
        }
    }

    /// Register a new system in a specific phase
    pub fn register_system<Phase: 'static>(&mut self, system: System) {
        let r#type = TypeId::of::<Phase>();
        match self.get_phase_mut(r#type) {
            Some(phase) => phase.add_system(system),
            None => {
                let mut phase = systems::Phase::new();
                phase.add_system(system);
                self.phases.push((r#type, phase));
            }
        }
    }
}
