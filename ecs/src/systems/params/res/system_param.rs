use crate::{Res, SystemParam, SystemParamState, UnsafeWorldCell, World, WorldAccess};

impl<'res, Resource: 'static> SystemParam for Res<'res, Resource> {
    type Item<'world, 'state> = Res<'world, Resource>;
    type State = ResState;

    fn create_state(world: &mut World) -> Self::State {
        ResState(world.get_or_reserve_resource_index::<Resource>())
    }

    fn access(access: &mut WorldAccess) {
        access.read_resource::<Resource>();
    }

    unsafe fn retrieve<'world, 'state>(
        state: &'state mut Self::State,
        world: &'world UnsafeWorldCell,
    ) -> Self::Item<'world, 'state> {
        unsafe { world.get().get_resource_at_index(state.0) }
    }
}

impl<'res, Resource: 'static> SystemParam for Option<Res<'res, Resource>> {
    type Item<'world, 'state> = Option<Res<'world, Resource>>;
    type State = ResState;

    fn create_state(world: &mut World) -> Self::State {
        ResState(world.get_or_reserve_resource_index::<Resource>())
    }

    fn access(access: &mut WorldAccess) {
        access.read_resource::<Resource>();
    }

    unsafe fn retrieve<'world, 'state>(
        state: &'state mut Self::State,
        world: &'world UnsafeWorldCell,
    ) -> Self::Item<'world, 'state> {
        unsafe { world.get().try_get_resource_at_index(state.0) }
    }
}

pub struct ResState(usize);

impl SystemParamState for ResState {}
