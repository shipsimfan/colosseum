use crate::Archetypes;

mod state;
mod system_param;

pub use state::*;
pub use system_param::*;

/// Query for entities in the world
pub struct Query<'world, 'state> {
    /// The state of the query
    state: &'state QueryState,

    /// Access to the archetypes of the world
    archetypes: &'world mut Archetypes,

    /// The current archetype being queried
    archetype: usize,

    /// The current index within the archetype being queried
    index: usize,
}
