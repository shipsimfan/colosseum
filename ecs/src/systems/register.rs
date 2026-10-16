use crate::{PhaseName, System, Systems, systems};

impl Systems {
    /// Register a new phase of systems
    pub fn register_phase<Phase: PhaseName>(&mut self, phase: Phase) {
        if self.get_phase(&phase).is_none() {
            self.phases.push((Box::new(phase), systems::Phase::new()));
        }
    }

    /// Register a new system in a specific phase
    pub fn register_system(&mut self, phase: Box<dyn PhaseName>, system: Box<dyn System>) {
        match self.get_phase_mut(phase.as_ref()) {
            Some(phase) => phase.add_system(system),
            None => {
                let mut new_phase = systems::Phase::new();
                new_phase.add_system(system);
                self.phases.push((phase, new_phase));
            }
        }
    }
}
