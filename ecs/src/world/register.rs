use crate::{System, World};

impl World {
    /// Register a new phase of systems
    pub fn register_phase<Phase: 'static>(&mut self) {
        self.systems.register_phase::<Phase>()
    }

    /// Register a new system into a phase
    pub fn register_system<Phase: 'static>(&mut self, system: System) {
        self.systems.register_system::<Phase>(system)
    }
}
