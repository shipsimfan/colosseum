use crate::World;
use colosseum_core::Result;

impl World {
    /// Run a specific phase of systems in the world
    pub fn run<Phase: 'static>(&mut self) -> Result<()> {
        self.systems
            .run::<Phase>(&mut self.resources, &mut self.archetypes)
    }
}
