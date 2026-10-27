use crate::{Entity, World};
use alexandria::Id;

/// A set of components that can be added to an entity in the ECS system
pub trait ComponentList: 'static {
    /// Add the components in this list to `entity`
    fn add_to_entity(self, entity: Id<Entity>, world: &mut World);
}

impl<T1: 'static + Send + Sync + Sized> ComponentList for (T1,) {
    fn add_to_entity(self, entity: Id<Entity>, world: &mut World) {
        world.add_component(entity, self.0);
    }
}

impl<T1: 'static + Send + Sync + Sized, T2: 'static + Send + Sync + Sized> ComponentList
    for (T1, T2)
{
    fn add_to_entity(self, entity: Id<Entity>, world: &mut World) {
        world.add_component(entity, self.0);
        world.add_component(entity, self.1);
    }
}

impl<
    T1: 'static + Send + Sync + Sized,
    T2: 'static + Send + Sync + Sized,
    T3: 'static + Send + Sync + Sized,
> ComponentList for (T1, T2, T3)
{
    fn add_to_entity(self, entity: Id<Entity>, world: &mut World) {
        world.add_component(entity, self.0);
        world.add_component(entity, self.1);
        world.add_component(entity, self.2);
    }
}

impl<
    T1: 'static + Send + Sync + Sized,
    T2: 'static + Send + Sync + Sized,
    T3: 'static + Send + Sync + Sized,
    T4: 'static + Send + Sync + Sized,
> ComponentList for (T1, T2, T3, T4)
{
    fn add_to_entity(self, entity: Id<Entity>, world: &mut World) {
        world.add_component(entity, self.0);
        world.add_component(entity, self.1);
        world.add_component(entity, self.2);
        world.add_component(entity, self.3);
    }
}

impl<
    T1: 'static + Send + Sync + Sized,
    T2: 'static + Send + Sync + Sized,
    T3: 'static + Send + Sync + Sized,
    T4: 'static + Send + Sync + Sized,
    T5: 'static + Send + Sync + Sized,
> ComponentList for (T1, T2, T3, T4, T5)
{
    fn add_to_entity(self, entity: Id<Entity>, world: &mut World) {
        world.add_component(entity, self.0);
        world.add_component(entity, self.1);
        world.add_component(entity, self.2);
        world.add_component(entity, self.3);
        world.add_component(entity, self.4);
    }
}

impl<
    T1: 'static + Send + Sync + Sized,
    T2: 'static + Send + Sync + Sized,
    T3: 'static + Send + Sync + Sized,
    T4: 'static + Send + Sync + Sized,
    T5: 'static + Send + Sync + Sized,
    T6: 'static + Send + Sync + Sized,
> ComponentList for (T1, T2, T3, T4, T5, T6)
{
    fn add_to_entity(self, entity: Id<Entity>, world: &mut World) {
        world.add_component(entity, self.0);
        world.add_component(entity, self.1);
        world.add_component(entity, self.2);
        world.add_component(entity, self.3);
        world.add_component(entity, self.4);
        world.add_component(entity, self.5);
    }
}

impl<
    T1: 'static + Send + Sync + Sized,
    T2: 'static + Send + Sync + Sized,
    T3: 'static + Send + Sync + Sized,
    T4: 'static + Send + Sync + Sized,
    T5: 'static + Send + Sync + Sized,
    T6: 'static + Send + Sync + Sized,
    T7: 'static + Send + Sync + Sized,
> ComponentList for (T1, T2, T3, T4, T5, T6, T7)
{
    fn add_to_entity(self, entity: Id<Entity>, world: &mut World) {
        world.add_component(entity, self.0);
        world.add_component(entity, self.1);
        world.add_component(entity, self.2);
        world.add_component(entity, self.3);
        world.add_component(entity, self.4);
        world.add_component(entity, self.5);
        world.add_component(entity, self.6);
    }
}

impl<
    T1: 'static + Send + Sync + Sized,
    T2: 'static + Send + Sync + Sized,
    T3: 'static + Send + Sync + Sized,
    T4: 'static + Send + Sync + Sized,
    T5: 'static + Send + Sync + Sized,
    T6: 'static + Send + Sync + Sized,
    T7: 'static + Send + Sync + Sized,
    T8: 'static + Send + Sync + Sized,
> ComponentList for (T1, T2, T3, T4, T5, T6, T7, T8)
{
    fn add_to_entity(self, entity: Id<Entity>, world: &mut World) {
        world.add_component(entity, self.0);
        world.add_component(entity, self.1);
        world.add_component(entity, self.2);
        world.add_component(entity, self.3);
        world.add_component(entity, self.4);
        world.add_component(entity, self.5);
        world.add_component(entity, self.6);
        world.add_component(entity, self.7);
    }
}
