use crate::{Commands, IntoSystem, PhaseName};

impl Commands {
    /// Register a new system to a phase
    pub fn register_system<
        Phase: PhaseName,
        Input: 'static,
        System: 'static + IntoSystem<Input>,
    >(
        &mut self,
        phase: Phase,
        system: System,
    ) {
        self.register_system_direct(phase, system.into_system());
    }

    /// Register a new top-level system to a phase directly
    pub fn register_system_direct<Phase: PhaseName, System: 'static + crate::System>(
        &mut self,
        phase: Phase,
        system: System,
    ) {
        self.systems_to_register
            .push((Box::new(phase), Box::new(system)));
    }
}
