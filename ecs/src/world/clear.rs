use crate::{Archetypes, PhaseName, Systems, World};

impl World {
    /// Clear all the systems from a single phase
    pub fn clear_phase<Phase: PhaseName>(&mut self, phase: Phase) {
        self.systems.clear_phase(phase)
    }

    /// Clear all systems, entities, and non-permanent resources from the world
    pub unsafe fn clear(&mut self) {
        self.archetypes = Archetypes::new();
        self.systems = Systems::new();
        self.resources.clear();
    }
}
