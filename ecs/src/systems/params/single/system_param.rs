use crate::{
    Query, QueryData, QueryFilter, QueryState, Single, SystemParam, UnsafeWorldCell, World,
    WorldAccess,
};
use std::marker::PhantomData;

impl<'world, 'state, Data: 'static + QueryData, Filter: 'static + QueryFilter> SystemParam
    for Single<'world, 'state, Data, Filter>
{
    type Item<'new_world, 'new_state> = Single<'new_world, 'new_state, Data, Filter>;

    type State = QueryState<Data, Filter>;

    fn create_state(_: &mut World) -> Self::State {
        QueryState::new()
    }

    fn access(access: &mut WorldAccess) {
        Data::access(access);
    }

    unsafe fn retrieve<'new_world, 'new_state>(
        state: &'new_state mut Self::State,
        world: UnsafeWorldCell<'new_world>,
    ) -> Self::Item<'new_world, 'new_state> {
        match unsafe { Query::<Data, Filter>::retrieve(state, world) }.next() {
            Some(value) => Single {
                value,
                _marker: PhantomData,
            },
            None => panic!("query returned no results"),
        }
    }
}

impl<'world, 'state, Data: 'static + QueryData, Filter: 'static + QueryFilter> SystemParam
    for Option<Single<'world, 'state, Data, Filter>>
{
    type Item<'new_world, 'new_state> = Option<Single<'new_world, 'new_state, Data, Filter>>;

    type State = QueryState<Data, Filter>;

    fn create_state(_: &mut World) -> Self::State {
        QueryState::new()
    }

    fn access(access: &mut WorldAccess) {
        Data::access(access);
    }

    unsafe fn retrieve<'new_world, 'new_state>(
        state: &'new_state mut Self::State,
        world: UnsafeWorldCell<'new_world>,
    ) -> Self::Item<'new_world, 'new_state> {
        unsafe { Query::<Data, Filter>::retrieve(state, world) }
            .next()
            .map(|value| Single {
                value,
                _marker: PhantomData,
            })
    }
}
