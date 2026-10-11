use crate::Systems;
use std::any::TypeId;

impl Systems {
    /// Remove a phase of systems
    pub fn remove_phase<T: 'static>(&mut self) {
        let r#type = TypeId::of::<T>();
        for i in 0..self.phases.len() {
            if self.phases[i].0 == r#type {
                self.phases.swap_remove(i);
                break;
            }
        }
    }
}
