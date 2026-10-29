use crate::{
    Query, QueryData, QueryFilter, QueryState, SystemParam, UnsafeWorldCell, World, WorldAccess,
};
use std::marker::PhantomData;

impl<'world, 'state, Data: 'static + QueryData, Filter: 'static + QueryFilter> SystemParam
    for Query<'world, 'state, Data, Filter>
{
    type Item<'new_world, 'new_state> = Query<'new_world, 'new_state, Data, Filter>;
    type State = QueryState<Data, Filter>;

    fn create_state(_: &mut World) -> Self::State {
        QueryState::new()
    }

    fn access(access: &mut WorldAccess) {
        Data::access(access);
    }

    unsafe fn retrieve<'new_world, 'new_state>(
        state: &'new_state mut Self::State,
        world: &'new_world UnsafeWorldCell,
    ) -> Self::Item<'new_world, 'new_state> {
        Query {
            state,
            world,
            archetype: 0,
            index: 0,
            _marker: PhantomData,
        }
    }
}
