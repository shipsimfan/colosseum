use crate::{Command, IntoSystem, PhaseName, RegisterSystem, World};

impl<Phase: PhaseName, Input: 'static, System: 'static + IntoSystem<Input>> Command
    for RegisterSystem<Phase, Input, System>
{
    fn apply(self: Box<Self>, world: &mut World) {
        world.register_system(self.phase, self.system);
    }
}
