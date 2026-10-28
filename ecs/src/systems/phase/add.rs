use crate::{Archetypes, systems::Phase};

impl Phase {
    /// Add a new system to the phase
    pub fn add_system<System: 'static + crate::System>(
        &mut self,
        mut system: System,
        archetypes: &Archetypes,
    ) {
        if system.deferred() {
            self.deferred.push(self.systems.len());
        }

        let access = system.access();
        if access.archetypes_needed() {
            self.archetypes_needed.push(self.systems.len());

            for (archetype, components) in archetypes.archetypes() {
                system.register_archetype(archetype, components);
            }
        }

        self.systems.push(Box::new(system));
    }
}
