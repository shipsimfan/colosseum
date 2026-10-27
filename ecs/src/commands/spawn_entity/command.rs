use crate::{Command, ComponentList, SpawnEntity, World};

impl<Components: 'static + ComponentList> Command for SpawnEntity<Components> {
    fn apply(self: Box<Self>, world: &mut World) {
        world.spawn(self.components);
    }
}
