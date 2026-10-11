use crate::Systems;
use std::any::TypeId;

impl Systems {
    /// Clear all the systems from a single phase
    pub fn clear_phase<Phase: 'static>(&mut self) {
        if let Some(phase) = self.get_phase_mut(TypeId::of::<Phase>()) {
            phase.clear();
        }
    }
}
