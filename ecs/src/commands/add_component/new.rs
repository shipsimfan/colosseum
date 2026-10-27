use crate::{AddComponent, Entity};
use alexandria::Id;

impl<Component: 'static + Send + Sync + Sized> AddComponent<Component> {
    /// Create a new [`AddComponent`] command
    pub fn new(entity: Id<Entity>, component: Component) -> AddComponent<Component> {
        AddComponent { entity, component }
    }
}
