use crate::{Command, RemoveComponent, World};

impl<Component: 'static + Send + Sync + Sized> Command for RemoveComponent<Component> {
    fn apply(self: Box<Self>, world: &mut World) {
        world.remove_component::<Component>(self.entity);
    }
}
