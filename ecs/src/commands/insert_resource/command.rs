use crate::{Command, InsertResource, World};

impl<Resource: 'static> Command for InsertResource<Resource> {
    fn apply(self: Box<Self>, world: &mut World) {
        world.insert_resource(self.resource);
    }
}
