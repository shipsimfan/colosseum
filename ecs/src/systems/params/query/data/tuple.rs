use crate::{QueryData, UnsafeWorldCell, WorldAccess};
use std::any::TypeId;

macro_rules! impl_tuple_query_data {
    ($($params: ident),*$(,)?) => {
        impl<$($params: QueryData),*> QueryData for ($($params,)*) {
            type Item<'world, 'state> = ($($params::Item<'world, 'state>,)*);
            type State = ($($params::State,)*);

            #[allow(unused_variables)]
            fn access(access: &mut WorldAccess) {
                $($params::access(access);)*
            }

            #[allow(unused_variables, non_snake_case)]
            fn create_state(components: &[TypeId]) -> Option<Self::State> {
                $(let $params = match $params::create_state(components) {
                    Some(state) => state,
                    None => return None,
                };)*

                Some(($($params,)*))
            }

            #[allow(unused_variables, non_snake_case)]
            unsafe fn query<'world, 'state>(
                archetype: usize,
                index: usize,
                world: &'world UnsafeWorldCell,
                state: &'state Self::State,
            ) -> Self::Item<'world, 'state> {
                let ($($params,)*) = state;
                ($(unsafe { $params::query(archetype, index, world, $params) },)*)
            }
        }
    };
}

impl_tuple_query_data!();
impl_tuple_query_data!(T1);
impl_tuple_query_data!(T1, T2);
impl_tuple_query_data!(T1, T2, T3);
impl_tuple_query_data!(T1, T2, T3, T4);
impl_tuple_query_data!(T1, T2, T3, T4, T5);
impl_tuple_query_data!(T1, T2, T3, T4, T5, T6);
impl_tuple_query_data!(T1, T2, T3, T4, T5, T6, T7);
impl_tuple_query_data!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_tuple_query_data!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_tuple_query_data!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_tuple_query_data!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_tuple_query_data!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_tuple_query_data!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
impl_tuple_query_data!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
impl_tuple_query_data!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15
);
impl_tuple_query_data!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16
);
