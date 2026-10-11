use crate::{Systems, systems::Phase};
use std::any::TypeId;

impl Systems {
    /// Get a reference to a phase of systems by its type, if it exists
    pub(in crate::systems) fn get_phase(&self, r#type: TypeId) -> Option<&Phase> {
        for (phase_type, phase) in &self.phases {
            if *phase_type == r#type {
                return Some(phase);
            }
        }

        None
    }

    /// Get a mutable reference to a phase of systems by its type, if it exists
    pub(in crate::systems) fn get_phase_mut(&mut self, r#type: TypeId) -> Option<&mut Phase> {
        for (phase_type, phase) in &mut self.phases {
            if *phase_type == r#type {
                return Some(phase);
            }
        }

        None
    }
}
