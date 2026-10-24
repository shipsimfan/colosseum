use crate::Resources;

impl Resources {
    /// Clear all non-permanent resources from the ECS system
    pub fn clear(&mut self) {
        self.resources.retain(|slot| slot.permanent());
    }
}
