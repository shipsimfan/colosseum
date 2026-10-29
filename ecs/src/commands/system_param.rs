use crate::{Commands, SystemParam, SystemParamState, UnsafeWorldCell, World, WorldAccess};

impl SystemParam for &mut Commands {
    type Item<'world, 'state> = &'state mut Commands;
    type State = Commands;

    const DEFERRED: bool = true;

    fn create_state(_: &mut World) -> Self::State {
        Commands {
            commands: Vec::new(),
        }
    }

    fn access(_: &mut WorldAccess) {}

    unsafe fn retrieve<'world, 'state>(
        state: &'state mut Self::State,
        _: &'world UnsafeWorldCell,
    ) -> Self::Item<'world, 'state> {
        state
    }
}

impl SystemParamState for Commands {
    fn apply(&mut self, world: &mut World) {
        for command in self.commands.drain(..) {
            command.apply(world);
        }
    }
}
