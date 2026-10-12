use crate::{FunctionSystem, IntoSystem, SystemParam, SystemReturn};
use std::marker::PhantomData;

macro_rules! impl_fn_into_system {
    ($($params: ident),*$(,)?) => {
        impl<$($params: SystemParam,)* R: SystemReturn, F: FnMut($($params),*) -> R> IntoSystem<($($params,)*)> for F
        where
            for<'a, 'b> &'a mut F: FnMut($($params),*) -> R
                + FnMut($(<$params as SystemParam>::Item<'b>),*) -> R,
        {
            type System = FunctionSystem<($($params,)*), F>;

            fn into_system(self) -> Self::System {
                FunctionSystem {
                    f: self,
                    _marker: PhantomData,
                }
            }
        }
    };
}

impl_fn_into_system!();
impl_fn_into_system!(T1);
impl_fn_into_system!(T1, T2);
impl_fn_into_system!(T1, T2, T3);
impl_fn_into_system!(T1, T2, T3, T4);
impl_fn_into_system!(T1, T2, T3, T4, T5);
impl_fn_into_system!(T1, T2, T3, T4, T5, T6);
impl_fn_into_system!(T1, T2, T3, T4, T5, T6, T7);
impl_fn_into_system!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_fn_into_system!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_fn_into_system!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_fn_into_system!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_fn_into_system!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_fn_into_system!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
impl_fn_into_system!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
impl_fn_into_system!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15
);
impl_fn_into_system!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16
);
