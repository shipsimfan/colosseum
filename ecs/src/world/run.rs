use crate::{PhaseName, World};
use colosseum_core::Result;

impl World {
    /// Run a specific phase of systems in the world
    pub fn run<Phase: PhaseName>(&mut self, phase: Phase) -> Result<()> {
        self.systems.run(phase, self.unsafe_cell())
    }
}
