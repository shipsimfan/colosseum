use crate::{SystemParam, UnsafeWorldCell, World, WorldAccess};

impl SystemParam for &mut World {
    type Item<'world, 'state> = &'world mut World;
    type State = ();

    fn create_state(_: &mut World) -> Self::State {
        ()
    }

    fn access(access: &mut WorldAccess) {
        access.set_exclusive();
    }

    unsafe fn retrieve<'world, 'state>(
        _: &'state mut Self::State,
        world: &'world UnsafeWorldCell,
    ) -> Self::Item<'world, 'state> {
        unsafe { world.get_mut() }
    }
}
