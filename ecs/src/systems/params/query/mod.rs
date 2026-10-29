use crate::UnsafeWorldCell;
use std::marker::PhantomData;

mod data;
mod filter;
mod state;

mod iterator;
mod system_param;

pub use data::*;
pub use filter::*;
pub use state::*;

/// Query for entities in the world
pub struct Query<'world, 'state, Components: QueryData, Filter: QueryFilter = ()> {
    /// The state of the query
    state: &'state QueryState<Components, Filter>,

    /// Access to the world
    world: &'world UnsafeWorldCell,

    /// The current archetype being queried
    archetype: usize,

    /// The current index within the archetype being queried
    index: usize,

    /// A marker for the components being queried
    _marker: PhantomData<fn() -> Components>,
}
