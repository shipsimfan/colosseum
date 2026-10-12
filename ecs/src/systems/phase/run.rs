use crate::{Archetypes, Resources, systems::Phase};
use colosseum_core::Result;

impl Phase {
    /// Run all systems in the phase
    pub fn run(&mut self, resources: &mut Resources, archetypes: &mut Archetypes) -> Result<()> {
        for system in &mut self.systems {
            system.run(resources, archetypes)?;
        }

        Ok(())
    }
}
