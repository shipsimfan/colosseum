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

    fn apply(state: &mut Self::State, world: &mut World) {
        for command in state.commands.drain(..) {
            command.apply(world);
        }
    }
}

impl SystemParamState for Commands {}
