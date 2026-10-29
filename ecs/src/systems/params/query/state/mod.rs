use crate::{QueryData, QueryFilter};
use std::marker::PhantomData;

mod get;
mod new;
mod system_param;

/// The state of a query system parameter
pub struct QueryState<Components: QueryData, Filter: QueryFilter> {
    /// The indices of the archetypes that match the query, along with the state needed to query that archetype
    archetypes: Vec<(usize, Components::State)>,

    /// A marker for the filter being used
    _filter: PhantomData<fn() -> Filter>,
}
