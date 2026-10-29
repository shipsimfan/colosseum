use crate::World;
use std::any::TypeId;

/// The state associated with a system parameter
pub trait SystemParamState {
    /// Registers the archetype with the system parameter state
    #[allow(unused_variables)]
    fn register_archetype(&mut self, archetype: usize, components: &[TypeId]) {}

    /// Applies the deferred system parameter to the given world
    #[allow(unused_variables)]
    fn apply(&mut self, world: &mut World) {}
}

macro_rules! impl_tuple_system_param_state {
    ($($params: ident),*$(,)?) => {
        impl<$($params: SystemParamState),*> SystemParamState for ($($params,)*) {
            #[allow(unused_variables, non_snake_case)]
            fn register_archetype(&mut self, archetype: usize, components: &[TypeId]) {
                let ($($params,)*) = self;
                $(
                    $params.register_archetype(archetype, components);
                )*
            }

            #[allow(unused_variables, non_snake_case)]
            fn apply(&mut self, world: &mut World) {
                let ($($params,)*) = self;
                $(
                    $params.apply(world);
                )*
            }
        }
    };
}

impl_tuple_system_param_state!();
impl_tuple_system_param_state!(T1);
impl_tuple_system_param_state!(T1, T2);
impl_tuple_system_param_state!(T1, T2, T3);
impl_tuple_system_param_state!(T1, T2, T3, T4);
impl_tuple_system_param_state!(T1, T2, T3, T4, T5);
impl_tuple_system_param_state!(T1, T2, T3, T4, T5, T6);
impl_tuple_system_param_state!(T1, T2, T3, T4, T5, T6, T7);
impl_tuple_system_param_state!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_tuple_system_param_state!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_tuple_system_param_state!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_tuple_system_param_state!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_tuple_system_param_state!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_tuple_system_param_state!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
impl_tuple_system_param_state!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
impl_tuple_system_param_state!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15
);
impl_tuple_system_param_state!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16
);
