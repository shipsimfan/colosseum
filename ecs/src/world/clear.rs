use crate::World;

impl World {
    /// Clear all the systems from a single phase
    pub fn clear_phase<Phase: 'static>(&mut self) {
        self.systems.clear_phase::<Phase>()
    }
}
