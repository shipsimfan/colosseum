use crate::{Commands, PhaseName};

impl Commands {
    /// Clear all the systems from a single phase
    pub fn clear_phase<Phase: PhaseName>(&mut self, phase: Phase) {
        self.phases_to_clear.push(Box::new(phase));
    }
}
