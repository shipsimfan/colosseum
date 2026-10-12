use crate::{System, systems::Phase};

impl Phase {
    /// Add a new system to the phase
    pub fn add_system(&mut self, system: Box<dyn System>) {
        self.systems.push(system);
    }
}
