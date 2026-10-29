use crate::QueryFilter;
use std::{any::TypeId, marker::PhantomData};

/// A filter that matches if any of its sub-filters match
pub struct Or<Filters> {
    /// A marker for the sub-filters
    _marker: PhantomData<fn() -> Filters>,
}

macro_rules! impl_or_filter {
    ($($params: ident),*$(,)?) => {
        impl<$($params: 'static),*> QueryFilter for Or<($($params,)*)> {
            #[allow(unused_variables)]
            fn filter(components: &[TypeId]) -> bool {
                $(components.contains(&TypeId::of::<$params>()) || )* false
            }
        }
    };
}

impl_or_filter!();
impl_or_filter!(T1);
impl_or_filter!(T1, T2);
impl_or_filter!(T1, T2, T3);
impl_or_filter!(T1, T2, T3, T4);
impl_or_filter!(T1, T2, T3, T4, T5);
impl_or_filter!(T1, T2, T3, T4, T5, T6);
impl_or_filter!(T1, T2, T3, T4, T5, T6, T7);
impl_or_filter!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_or_filter!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_or_filter!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_or_filter!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_or_filter!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_or_filter!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
impl_or_filter!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
impl_or_filter!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15
);
impl_or_filter!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16
);
