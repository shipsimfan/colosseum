use crate::Commands;

mod command;

/// Create a new entity
pub struct CreateEntity;

impl Commands {
    /// Create a new entity
    pub fn create_entity(&mut self) {
        self.queue(CreateEntity);
    }
}
