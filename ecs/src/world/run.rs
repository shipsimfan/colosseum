use crate::{PhaseName, World};
use colosseum_core::Result;

impl World {
    /// Run a specific phase of systems in the world
    pub fn run<Phase: PhaseName>(&mut self, phase: Phase) -> Result<()> {
        let r#unsafe = self.unsafe_cell();

        unsafe { r#unsafe.get_mut() }
            .systems
            .run(phase, r#unsafe.clone())
    }
}
