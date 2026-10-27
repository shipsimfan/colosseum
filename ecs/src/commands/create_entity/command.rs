use crate::{Command, CreateEntity, World};

impl Command for CreateEntity {
    fn apply(self: Box<Self>, world: &mut World) {
        world.create_entity();
    }
}
