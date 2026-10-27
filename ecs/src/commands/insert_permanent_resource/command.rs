use crate::{Command, InsertPermanentResource, World};

impl<Resource: 'static> Command for InsertPermanentResource<Resource> {
    fn apply(self: Box<Self>, world: &mut World) {
        world.insert_permanent_resource(self.resource);
    }
}
