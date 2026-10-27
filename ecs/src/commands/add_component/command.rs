use crate::{AddComponent, Command, World};

impl<Component: 'static + Send + Sync + Sized> Command for AddComponent<Component> {
    fn apply(self: Box<Self>, world: &mut World) {
        world.add_component(self.entity, self.component);
    }
}
