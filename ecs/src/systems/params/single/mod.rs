use crate::{QueryData, QueryFilter};
use std::marker::PhantomData;

mod deref;
mod system_param;

/// A query which returns a single component matching the specified filter
pub struct Single<'world, 'state, Data: QueryData, Filter: QueryFilter = ()> {
    /// The value to be returned
    value: Data::Item<'world, 'state>,

    /// A marker for the components being queried and the filter
    _marker: PhantomData<fn() -> (Data, Filter)>,
}
