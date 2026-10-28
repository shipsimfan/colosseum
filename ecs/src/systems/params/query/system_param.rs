use crate::{Query, QueryState, SystemParam, UnsafeWorldCell, World, WorldAccess};

impl<'world, 'state> SystemParam for Query<'world, 'state> {
    type Item<'new_world, 'new_state> = Query<'new_world, 'new_state>;
    type State = QueryState;

    fn create_state(world: &mut World) -> Self::State {
        QueryState::new()
    }

    fn access(access: &mut WorldAccess) {
        todo!("Add component accesses");
    }

    unsafe fn retrieve<'new_world, 'new_state>(
        state: &'new_state mut Self::State,
        world: &'new_world UnsafeWorldCell,
    ) -> Self::Item<'new_world, 'new_state> {
        Query {
            state,
            archetypes: unsafe { world.get_mut().archetypes_mut() },
            archetype: 0,
            index: 0,
        }
    }
}
