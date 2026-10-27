use crate::{AddComponents, Command, ComponentList, World};

impl<Components: ComponentList> Command for AddComponents<Components> {
    fn apply(self: Box<Self>, world: &mut World) {
        world.add_components(self.entity, self.components);
    }
}
