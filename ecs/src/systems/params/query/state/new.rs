use crate::{QueryData, QueryFilter, QueryState};
use std::marker::PhantomData;

impl<Data: QueryData, Filter: QueryFilter> QueryState<Data, Filter> {
    /// Create a new [`QueryState`]
    pub(in crate::systems::params::query) fn new() -> QueryState<Data, Filter> {
        QueryState {
            archetypes: Vec::new(),
            _filter: PhantomData,
        }
    }
}
