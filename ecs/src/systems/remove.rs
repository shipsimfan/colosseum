use crate::{PhaseName, Systems};

impl Systems {
    /// Remove a phase of systems
    pub fn remove_phase<Phase: PhaseName>(&mut self, phase: Phase) {
        for i in 0..self.phases.len() {
            if self.phases[i].0.dyn_eq(&phase) {
                self.phases.swap_remove(i);
                break;
            }
        }
    }
}
