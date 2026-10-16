use crate::{Res, Resources, SystemParam, UnsafeWorldCell, WorldAccess};

impl<'res, Resource: 'static> SystemParam for Res<'res, Resource> {
    type Item<'world> = Res<'world, Resource>;
    type State = usize;

    fn create_state(resources: &mut Resources) -> Self::State {
        resources.get_or_reserve_index::<Resource>()
    }

    fn access(access: &mut WorldAccess) {
        access.read_resource::<Resource>();
    }

    unsafe fn retrieve<'world>(
        world: &'world UnsafeWorldCell,
        state: &Self::State,
    ) -> Self::Item<'world> {
        unsafe { world.get().get_resource_at_index(*state) }
    }
}

impl<'res, Resource: 'static> SystemParam for Option<Res<'res, Resource>> {
    type Item<'world> = Option<Res<'world, Resource>>;
    type State = usize;

    fn create_state(resources: &mut Resources) -> Self::State {
        resources.get_or_reserve_index::<Resource>()
    }

    fn access(access: &mut WorldAccess) {
        access.read_resource::<Resource>();
    }

    unsafe fn retrieve<'world>(
        world: &'world UnsafeWorldCell,
        state: &Self::State,
    ) -> Self::Item<'world> {
        unsafe { world.get().try_get_resource_at_index(*state) }
    }
}
