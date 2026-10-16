use crate::{IntoSystem, PhaseName, System, World};

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
        self.register_boxed_system(Box::new(phase), Box::new(system.into_system()));
    }

    /// Register a boxed system into a phase
    pub fn register_boxed_system(
        &mut self,
        phase: Box<dyn PhaseName>,
        mut system: Box<dyn System>,
    ) {
        system.initialize(self);
        self.systems.register_system(phase, system)
    }
}
