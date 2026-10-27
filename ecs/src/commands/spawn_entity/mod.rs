use crate::{Commands, ComponentList};

mod command;
mod new;

/// Spawn a new entity with a set of components
pub struct SpawnEntity<Components: 'static + ComponentList> {
    /// The components to add to the new entity
    components: Components,
}

impl Commands {
    /// Spawn a new entity with a set of components
    pub fn spawn<Components: 'static + ComponentList>(&mut self, components: Components) {
        self.spawn_entity(components);
    }

    /// Spawn a new entity with a set of components
    pub fn spawn_entity<Components: 'static + ComponentList>(&mut self, components: Components) {
        self.queue(SpawnEntity::new(components))
    }
}
