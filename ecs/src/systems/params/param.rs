use crate::{SystemParamState, UnsafeWorldCell, World, WorldAccess};

/// A parameter that can be used by a system
pub trait SystemParam {
    /// The type of the system parameter when retrieved
    type Item<'world, 'state>;

    /// The state associated with this system parameter
    type State: SystemParamState;

    /// Indicates whether the system parameter is deferred
    const DEFERRED: bool = false;

    /// Creates a new state associated with this system parameter
    fn create_state(world: &mut World) -> Self::State;

    /// Add and check the required world access for this system parameter
    fn access(access: &mut WorldAccess);

    /// Retrieves the system parameter from the given resources and archetypes
    unsafe fn retrieve<'world, 'state>(
        state: &'state mut Self::State,
        world: &'world UnsafeWorldCell,
    ) -> Self::Item<'world, 'state>;
}

macro_rules! impl_tuple_system_param {
    ($($params: ident),*$(,)?) => {
        impl<$($params: SystemParam),*> SystemParam for ($($params,)*) {
            type Item<'world, 'state> = ($($params::Item<'world, 'state>,)*);
            type State = ($($params::State,)*);

            const DEFERRED: bool = $($params::DEFERRED || )* false;

            #[allow(unused_variables)]
            fn create_state(world: &mut World) -> Self::State {
                ($($params::create_state(world),)*)
            }

            #[allow(unused_variables)]
            fn access(access: &mut WorldAccess) {
                $(
                    $params::access(access);
                )*
            }

            #[allow(unused_variables, non_snake_case)]
            unsafe fn retrieve<'world, 'state>(
                state: &'state mut Self::State,
                world: &'world UnsafeWorldCell,
            ) -> Self::Item<'world, 'state> {
                let ($($params,)*) = state;
                ($(
                    unsafe { $params::retrieve($params, world) },
                )*)
            }
        }
    };
}

impl_tuple_system_param!();
impl_tuple_system_param!(T1);
impl_tuple_system_param!(T1, T2);
impl_tuple_system_param!(T1, T2, T3);
impl_tuple_system_param!(T1, T2, T3, T4);
impl_tuple_system_param!(T1, T2, T3, T4, T5);
impl_tuple_system_param!(T1, T2, T3, T4, T5, T6);
impl_tuple_system_param!(T1, T2, T3, T4, T5, T6, T7);
impl_tuple_system_param!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_tuple_system_param!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_tuple_system_param!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_tuple_system_param!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_tuple_system_param!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_tuple_system_param!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
impl_tuple_system_param!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
impl_tuple_system_param!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15
);
impl_tuple_system_param!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16
);
