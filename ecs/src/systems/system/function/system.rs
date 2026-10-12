use crate::{Archetypes, FunctionSystem, Resources, System, SystemParam, SystemReturn};
use colosseum_core::Result;

macro_rules! impl_fn_system {
    ($($params: ident),*$(,)?) => {
        impl<$($params: SystemParam,)* R: SystemReturn, F> System for FunctionSystem<($($params,)*), F>
        where
            for<'a, 'b> &'a mut F: FnMut($($params),*) -> R
                + FnMut($(<$params as SystemParam>::Item<'b>),*) -> R
        {
            #[allow(non_snake_case)]
            #[allow(unused_variables)]
            fn run(&mut self, resources: &mut Resources, archetypes: &mut Archetypes) -> Result<()> {
                fn call_inner<$($params,)* R: SystemReturn>(
                    mut f: impl FnMut($($params),*) -> R,
                    $($params: $params),*
                ) -> Result<()> {
                    f($($params),*).into_result()
                }

                $(
                    let $params = $params::retrieve(resources, archetypes);
                )*

                call_inner(&mut self.f, $($params),*)
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
