use crate::{ComponentList, SpawnEntity};

impl<Components: 'static + ComponentList> SpawnEntity<Components> {
    /// Create a new [`SpawnEntity`] command
    pub fn new(components: Components) -> SpawnEntity<Components> {
        SpawnEntity { components }
    }
}
