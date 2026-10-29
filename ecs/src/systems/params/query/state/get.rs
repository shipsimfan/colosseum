use crate::{QueryData, QueryFilter, QueryState};

impl<Components: QueryData, Filter: QueryFilter> QueryState<Components, Filter> {
    /// Get the state for a specific archetype by its index
    pub fn get(&self, index: usize) -> Option<(usize, &Components::State)> {
        self.archetypes
            .get(index)
            .map(|(index, state)| (*index, state))
    }
}
