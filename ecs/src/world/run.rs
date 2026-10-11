use crate::World;

impl World {
    /// Run a specific phase of systems in the world
    pub fn run<Phase: 'static>(&mut self) {
        self.systems
            .run::<Phase>(&mut self.resources, &mut self.archetypes);
    }
}
