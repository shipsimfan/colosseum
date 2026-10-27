use crate::systems::Phase;

impl Phase {
    /// Add a new system to the phase
    pub fn add_system<System: 'static + crate::System>(&mut self, system: System) {
        if system.deferred() {
            self.deferred.push(self.systems.len());
        }

        self.systems.push(Box::new(system));
    }
}
