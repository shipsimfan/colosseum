use crate::{QueryData, UnsafeWorldCell, WorldAccess};
use std::any::TypeId;

impl<T: QueryData> QueryData for Option<T> {
    type Item<'world, 'state> = Option<T::Item<'world, 'state>>;
    type State = Option<T::State>;

    fn access(access: &mut WorldAccess) {
        T::access(access);
    }

    fn create_state(components: &[TypeId]) -> Option<Self::State> {
        Some(T::create_state(components))
    }

    unsafe fn query<'world, 'state>(
        archetype: usize,
        index: usize,
        world: &'world UnsafeWorldCell,
        state: &'state Self::State,
    ) -> Self::Item<'world, 'state> {
        state
            .as_ref()
            .map(|state| unsafe { T::query(archetype, index, world, &state) })
    }
}
