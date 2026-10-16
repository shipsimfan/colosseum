use crate::{System, systems::Phase};

impl Phase {
    /// Add a new system to the phase
    pub fn add_system(&mut self, system: Box<dyn System>) {
        if system.deferred() {
            self.deferred.push(self.systems.len());
        }

        self.systems.push(system);
    }
}
