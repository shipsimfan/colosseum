use crate::{IntoSystem, PhaseName, Systems, systems};

impl Systems {
    /// Register a new phase of systems
    pub fn register_phase<Phase: PhaseName>(&mut self, phase: Phase) {
        if self.get_phase(&phase).is_none() {
            self.phases.push((Box::new(phase), systems::Phase::new()));
        }
    }

    /// Register a new system in a specific phase
    pub fn register_system<
        Phase: PhaseName,
        Input: 'static,
        System: 'static + IntoSystem<Input>,
    >(
        &mut self,
        phase: Phase,
        system: System,
    ) {
        let system: Box<dyn crate::System> = Box::new(system.into_system());
        match self.get_phase_mut(&phase) {
            Some(phase) => phase.add_system(system),
            None => {
                let mut new_phase = systems::Phase::new();
                new_phase.add_system(system);
                self.phases.push((Box::new(phase), new_phase));
            }
        }
    }
}
