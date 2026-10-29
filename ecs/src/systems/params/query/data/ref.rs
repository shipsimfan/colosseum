use crate::{QueryData, UnsafeWorldCell, WorldAccess};
use std::any::TypeId;

impl<T: 'static + Send + Sync + Sized> QueryData for &T {
    type Item<'world, 'state> = &'world T;
    type State = usize;

    fn access(access: &mut WorldAccess) {
        access.read_component::<T>();
    }

    fn create_state(components: &[TypeId]) -> Option<Self::State> {
        let r#type = TypeId::of::<T>();
        components.iter().position(|&id| id == r#type)
    }

    unsafe fn query<'world, 'state>(
        archetype: usize,
        index: usize,
        world: &'world UnsafeWorldCell,
        state: &'state Self::State,
    ) -> Self::Item<'world, 'state> {
        unsafe { world.get() }
            .archetypes()
            .get_at(archetype, index, *state)
    }
}
