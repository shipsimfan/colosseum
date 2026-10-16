use crate::{Resources, SystemParam, UnsafeWorldCell, World, WorldAccess};

impl SystemParam for &mut World {
    type Item<'new> = &'new mut World;
    type State = ();

    fn create_state(_: &mut Resources) -> Self::State {
        ()
    }

    fn access(access: &mut WorldAccess) {
        access.set_exclusive();
    }

    unsafe fn retrieve<'world>(
        world: &'world UnsafeWorldCell,
        _: &Self::State,
    ) -> Self::Item<'world> {
        unsafe { world.get_mut() }
    }
}
