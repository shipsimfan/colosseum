use crate::{Commands, SystemParam, UnsafeWorldCell, World, WorldAccess};

impl SystemParam for &mut Commands {
    type Item<'world, 'state> = &'state mut Commands;
    type State = Commands;

    const DEFERRED: bool = true;

    fn create_state(_: &mut World) -> Self::State {
        Commands {
            resources_to_insert: Vec::new(),
            resources_to_remove: Vec::new(),
            systems_to_register: Vec::new(),
            phases_to_clear: Vec::new(),
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
        for resource in state.resources_to_remove.drain(..) {
            world.remove_resource_by_type_id(resource);
        }

        for (r#type, resource) in state.resources_to_insert.drain(..) {
            world.insert_resource_by_type_id(r#type, resource);
        }

        for phase in state.phases_to_clear.drain(..) {
            world.clear_phase_boxed(phase);
        }

        for (phase, system) in state.systems_to_register.drain(..) {
            world.register_boxed_system(phase, system);
        }
    }
}
