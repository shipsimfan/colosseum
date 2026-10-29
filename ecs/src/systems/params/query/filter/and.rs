use crate::QueryFilter;
use std::{any::TypeId, marker::PhantomData};

/// A filter that matches if all of its sub-filters match
pub struct And<Filters> {
    /// A marker for the sub-filters
    _marker: PhantomData<fn() -> Filters>,
}

macro_rules! impl_and_filter {
    ($($params: ident),*$(,)?) => {
        impl<$($params: 'static),*> QueryFilter for And<($($params,)*)> {
            #[allow(unused_variables)]
            fn filter(components: &[TypeId]) -> bool {
                $(components.contains(&TypeId::of::<$params>()) && )* true
            }
        }

        impl<$($params: 'static),*> QueryFilter for ($($params,)*) {
            #[allow(unused_variables)]
            fn filter(components: &[TypeId]) -> bool {
                $(components.contains(&TypeId::of::<$params>()) && )* true
            }
        }
    };
}

impl_and_filter!();
impl_and_filter!(T1);
impl_and_filter!(T1, T2);
impl_and_filter!(T1, T2, T3);
impl_and_filter!(T1, T2, T3, T4);
impl_and_filter!(T1, T2, T3, T4, T5);
impl_and_filter!(T1, T2, T3, T4, T5, T6);
impl_and_filter!(T1, T2, T3, T4, T5, T6, T7);
impl_and_filter!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_and_filter!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_and_filter!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_and_filter!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_and_filter!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_and_filter!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
impl_and_filter!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
impl_and_filter!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15
);
impl_and_filter!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16
);
