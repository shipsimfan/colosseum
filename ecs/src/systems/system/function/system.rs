use crate::{
    FunctionSystem, IntoSystem, System, SystemParam, SystemParamState, SystemReturn,
    UnsafeWorldCell, World, WorldAccess,
};
use colosseum_core::Result;
use std::any::TypeId;

macro_rules! impl_fn_system {
    ($state_name: ident, $($params: ident),*$(,)?) => {
        /// The state for the function system containing the states of all system parameters
        #[allow(non_snake_case)]
        pub struct $state_name<$($params: SystemParam,)*> {$(
            $params: $params::State,
        )*}

        impl<$($params: SystemParam,)* R: SystemReturn, F> System for FunctionSystem<($($params,)*), $state_name<$($params,)*>, F>
        where
            for<'a, 'world, 'state> &'a mut F: FnMut($($params),*) -> R
                + FnMut($(<$params as SystemParam>::Item<'world, 'state>),*) -> R
        {
            #[allow(unused_variables)]
            fn initialize(&mut self, world: &mut World) {
                self.state = Some($state_name {$(
                    $params: $params::create_state(world),
                )*});
            }

            #[allow(non_snake_case)]
            #[allow(unused_variables)]
            fn run(&mut self, world: &UnsafeWorldCell) -> Result<()> {
                fn call_inner<$($params,)* R: SystemReturn>(
                    mut f: impl FnMut($($params),*) -> R,
                    $($params: $params),*
                ) -> Result<()> {
                    f($($params),*).into_result()
                }

                let state = self.state.as_mut().unwrap();

                $(
                    let $params = unsafe { $params::retrieve(&mut state.$params, world) };
                )*

                call_inner(&mut self.f, $($params),*)
            }

            fn deferred(&self) -> bool {
                $($params::DEFERRED ||)* false
            }

            fn access(&self) -> &WorldAccess {
                &self.access
            }

            #[allow(unused_variables)]
            fn register_archetype(&mut self, archetype: usize, components: &[TypeId]) {
                let state = self.state.as_mut().unwrap();

                $(
                    state.$params.register_archetype(archetype, components);
                )*
            }

            #[allow(unused_variables)]
            fn apply(&mut self, world: &mut World) {
                let state = self.state.as_mut().unwrap();

                $(
                    if $params::DEFERRED {
                        $params::apply(&mut state.$params, world);
                    }
                )*
            }
        }

        impl<$($params: SystemParam,)* R: SystemReturn, F: FnMut($($params),*) -> R> IntoSystem<($($params,)*)> for F
        where
            for<'a, 'world, 'state> &'a mut F: FnMut($($params),*) -> R
                + FnMut($(<$params as SystemParam>::Item<'world, 'state>),*) -> R,
        {
            type System = FunctionSystem<($($params,)*), $state_name<$($params,)*>, F>;

            #[allow(unused_variables)]
            fn into_system(self) -> Self::System {
                #[allow(unused_mut)]
                let mut access = WorldAccess::None;
                $(
                    $params::access(&mut access);
                )*

                FunctionSystem::new(self, access)
            }
        }
    };
}

impl_fn_system!(FunctionSystemState0,);
impl_fn_system!(FunctionSystemState1, T1);
impl_fn_system!(FunctionSystemState2, T1, T2);
impl_fn_system!(FunctionSystemState3, T1, T2, T3);
impl_fn_system!(FunctionSystemState4, T1, T2, T3, T4);
impl_fn_system!(FunctionSystemState5, T1, T2, T3, T4, T5);
impl_fn_system!(FunctionSystemState6, T1, T2, T3, T4, T5, T6);
impl_fn_system!(FunctionSystemState7, T1, T2, T3, T4, T5, T6, T7);
impl_fn_system!(FunctionSystemState8, T1, T2, T3, T4, T5, T6, T7, T8);
impl_fn_system!(FunctionSystemState9, T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_fn_system!(
    FunctionSystemState10,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9,
    T10
);
impl_fn_system!(
    FunctionSystemState11,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9,
    T10,
    T11
);
impl_fn_system!(
    FunctionSystemState12,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9,
    T10,
    T11,
    T12
);
impl_fn_system!(
    FunctionSystemState13,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9,
    T10,
    T11,
    T12,
    T13
);
impl_fn_system!(
    FunctionSystemState14,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9,
    T10,
    T11,
    T12,
    T13,
    T14
);
impl_fn_system!(
    FunctionSystemState15,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9,
    T10,
    T11,
    T12,
    T13,
    T14,
    T15
);
impl_fn_system!(
    FunctionSystemState16,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9,
    T10,
    T11,
    T12,
    T13,
    T14,
    T15,
    T16
);
