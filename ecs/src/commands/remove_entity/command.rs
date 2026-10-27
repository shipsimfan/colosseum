use crate::{Command, RemoveEntity, World};

impl Command for RemoveEntity {
    fn apply(self: Box<Self>, world: &mut World) {
        world.remove_entity(self.entity);
    }
}
