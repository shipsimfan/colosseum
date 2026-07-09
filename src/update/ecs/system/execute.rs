use crate::update::ecs::{Archetype, System};

impl System {
    /// Execute the system with the given archetypes
    pub(in crate::update::ecs) fn execute(&self, archetypes: &mut [Archetype]) {
        (self.system)(archetypes, &self.indices);
    }
}
