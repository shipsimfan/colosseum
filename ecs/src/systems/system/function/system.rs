use crate::{
    FunctionSystem, IntoSystem, System, SystemParam, SystemParamState, SystemReturn,
    UnsafeWorldCell, World, WorldAccess,
};
use colosseum_core::Result;
use std::any::TypeId;

macro_rules! impl_fn_system {
    ($($params: ident),*$(,)?) => {
        impl<$($params: SystemParam,)* R: SystemReturn, F> System for
            FunctionSystem<($($params,)*), ($($params::State,)*), F>
        where
            for<'a, 'world, 'state> &'a mut F: FnMut($($params),*) -> R
                + FnMut($(<$params as SystemParam>::Item<'world, 'state>),*) -> R
        {
            #[allow(unused_variables)]
            fn initialize(&mut self, world: &mut World) {
                self.state = Some(($($params::create_state(world),)*));
            }

            #[allow(unused_variables, non_snake_case)]
            fn run(&mut self, world: &UnsafeWorldCell) -> Result<()> {
                fn call_inner<$($params,)* R: SystemReturn>(
                    mut f: impl FnMut($($params),*) -> R,
                    $($params: $params),*
                ) -> Result<()> {
                    f($($params),*).into_result()
                }

                let ($($params,)*) = self.state.as_mut().unwrap();

                $(
                    let $params = unsafe { $params::retrieve($params, world) };
                )*

                call_inner(&mut self.f, $($params),*)
            }

            fn deferred(&self) -> bool {
                $($params::DEFERRED ||)* false
            }

            fn access(&self) -> &WorldAccess {
                &self.access
            }

            #[allow(unused_variables, non_snake_case)]
            fn register_archetype(&mut self, archetype: usize, components: &[TypeId]) {
                let ($($params,)*) = self.state.as_mut().unwrap();

                $(
                    $params.register_archetype(archetype, components);
                )*
            }

            #[allow(unused_variables, non_snake_case)]
            fn apply(&mut self, world: &mut World) {
                let ($($params,)*) = self.state.as_mut().unwrap();

                $(
                    if $params::DEFERRED {
                        $params.apply(world);
                    }
                )*
            }
        }

        impl<$($params: SystemParam,)* R: SystemReturn, F: FnMut($($params),*) -> R> IntoSystem<($($params,)*)> for F
        where
            for<'a, 'world, 'state> &'a mut F: FnMut($($params),*) -> R
                + FnMut($(<$params as SystemParam>::Item<'world, 'state>),*) -> R,
        {
            type System = FunctionSystem<($($params,)*), ($($params::State,)*), F>;

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

impl_fn_system!();
impl_fn_system!(T1);
impl_fn_system!(T1, T2);
impl_fn_system!(T1, T2, T3);
impl_fn_system!(T1, T2, T3, T4);
impl_fn_system!(T1, T2, T3, T4, T5);
impl_fn_system!(T1, T2, T3, T4, T5, T6);
impl_fn_system!(T1, T2, T3, T4, T5, T6, T7);
impl_fn_system!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_fn_system!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_fn_system!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_fn_system!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_fn_system!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_fn_system!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
impl_fn_system!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
impl_fn_system!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15
);
impl_fn_system!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16
);
