mod new;
mod system_param;

/// The state of a query system parameter
pub struct QueryState {
    /// The indices of the archetypes that match the query
    archetypes: Vec<usize>,

    /// The component indices within each archetype
    components: Vec<usize>,
}
