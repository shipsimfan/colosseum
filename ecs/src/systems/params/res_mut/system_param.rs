use crate::{ResMut, SystemParam, SystemParamState, UnsafeWorldCell, World, WorldAccess};

impl<'res, Resource: 'static> SystemParam for ResMut<'res, Resource> {
    type Item<'world, 'state> = ResMut<'world, Resource>;
    type State = ResMutState;

    fn create_state(world: &mut World) -> Self::State {
        ResMutState(world.get_or_reserve_resource_index::<Resource>())
    }

    fn access(access: &mut WorldAccess) {
        access.write_resource::<Resource>();
    }

    unsafe fn retrieve<'world, 'state>(
        state: &'state mut Self::State,
        world: &'world UnsafeWorldCell,
    ) -> Self::Item<'world, 'state> {
        unsafe { world.get_mut().get_resource_at_index_mut(state.0) }
    }
}

impl<'res, Resource: 'static> SystemParam for Option<ResMut<'res, Resource>> {
    type Item<'world, 'state> = Option<ResMut<'world, Resource>>;
    type State = ResMutState;

    fn create_state(world: &mut World) -> Self::State {
        ResMutState(world.get_or_reserve_resource_index::<Resource>())
    }

    fn access(access: &mut WorldAccess) {
        access.write_resource::<Resource>();
    }

    unsafe fn retrieve<'world, 'state>(
        state: &'state mut Self::State,
        world: &'world UnsafeWorldCell,
    ) -> Self::Item<'world, 'state> {
        unsafe { world.get_mut().try_get_resource_at_index_mut(state.0) }
    }
}

pub struct ResMutState(usize);

impl SystemParamState for ResMutState {}
