use crate::{ResMut, Resources, SystemParam, UnsafeWorldCell, WorldAccess};

impl<'res, Resource: 'static> SystemParam for ResMut<'res, Resource> {
    type Item<'new> = ResMut<'new, Resource>;
    type State = usize;

    fn create_state(resources: &mut Resources) -> Self::State {
        resources.get_or_reserve_index::<Resource>()
    }

    fn access(access: &mut WorldAccess) {
        access.write_resource::<Resource>();
    }

    unsafe fn retrieve<'world>(
        world: &'world UnsafeWorldCell,
        state: &Self::State,
    ) -> Self::Item<'world> {
        unsafe { world.get_mut().get_resource_at_index_mut(*state) }
    }
}

impl<'res, Resource: 'static> SystemParam for Option<ResMut<'res, Resource>> {
    type Item<'new> = Option<ResMut<'new, Resource>>;
    type State = usize;

    fn create_state(resources: &mut Resources) -> Self::State {
        resources.get_or_reserve_index::<Resource>()
    }

    fn access(access: &mut WorldAccess) {
        access.write_resource::<Resource>();
    }

    unsafe fn retrieve<'world>(
        world: &'world UnsafeWorldCell,
        state: &Self::State,
    ) -> Self::Item<'world> {
        unsafe { world.get_mut().try_get_resource_at_index_mut(*state) }
    }
}
