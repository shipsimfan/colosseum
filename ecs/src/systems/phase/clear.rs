use crate::systems::Phase;

impl Phase {
    /// Clear all systems from this phase
    pub fn clear(&mut self) {
        self.systems.clear();
    }
}
