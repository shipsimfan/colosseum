use crate::{IntoSystem, PhaseName, World};

impl World {
    /// Register a new phase of systems
    pub fn register_phase<Phase: PhaseName>(&mut self, phase: Phase) {
        self.systems.register_phase(phase)
    }

    /// Register a new system into a phase
    pub fn register_system<
        Phase: PhaseName,
        Input: 'static,
        System: 'static + IntoSystem<Input>,
    >(
        &mut self,
        phase: Phase,
        system: System,
    ) {
        self.systems.register_system(phase, system)
    }
}
