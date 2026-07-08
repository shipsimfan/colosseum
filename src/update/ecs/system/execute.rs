use crate::update::{System, ecs::Archetype};

impl System {
    /// Execute the system with the given archetypes
    pub(in crate::update::ecs) fn execute(&mut self, archetypes: &mut [Archetype]) {
        (self.system)(archetypes, &self.indices);
    }
}
