use crate::{UnsafeWorldCell, systems::Phase};
use colosseum_core::Result;

impl Phase {
    /// Run all systems in the phase
    pub fn run(&mut self, world: UnsafeWorldCell) -> Result<()> {
        for system in &mut self.systems {
            system.run(&world)?;
        }

        Ok(())
    }
}
