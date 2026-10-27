use crate::{Command, RemoveResource, World};

impl<Resource: 'static> Command for RemoveResource<Resource> {
    fn apply(self: Box<Self>, world: &mut World) {
        world.remove_resource::<Resource>();
    }
}
