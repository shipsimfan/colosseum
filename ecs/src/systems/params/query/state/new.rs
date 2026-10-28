use crate::QueryState;

impl QueryState {
    /// Create a new [`QueryState`]
    pub(in crate::systems::params::query) fn new() -> QueryState {
        QueryState {
            archetypes: Vec::new(),
            components: Vec::new(),
        }
    }
}
