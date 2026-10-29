use crate::{QueryData, UnsafeWorldCell, WorldAccess};
use std::any::TypeId;

impl<T: 'static + Send + Sync + Sized> QueryData for &mut T {
    type Item<'world, 'state> = &'world mut T;
    type State = usize;

    fn access(access: &mut WorldAccess) {
        access.write_component::<T>();
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
        unsafe { world.get_mut() }
            .archetypes_mut()
            .get_at_mut(archetype, index, *state)
    }
}
