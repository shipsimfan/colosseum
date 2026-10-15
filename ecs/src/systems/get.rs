use crate::{PhaseName, Systems, systems::Phase};

impl Systems {
    /// Get a reference to a phase of systems by its type, if it exists
    pub(in crate::systems) fn get_phase(&self, name: &dyn PhaseName) -> Option<&Phase> {
        for (phase_name, phase) in &self.phases {
            if phase_name.dyn_eq(name) {
                return Some(phase);
            }
        }

        None
    }

    /// Get a mutable reference to a phase of systems by its type, if it exists
    pub(in crate::systems) fn get_phase_mut(&mut self, name: &dyn PhaseName) -> Option<&mut Phase> {
        for (phase_name, phase) in &mut self.phases {
            if phase_name.dyn_eq(name) {
                return Some(phase);
            }
        }

        None
    }
}
