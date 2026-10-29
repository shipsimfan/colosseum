use crate::{UnsafeWorldCell, WorldAccess};
use std::any::TypeId;

mod r#mut;
mod option;
mod r#ref;
mod tuple;

/// An item that can be a data element in a query parameter
pub trait QueryData {
    /// The type of the query data when retrieved
    type Item<'world, 'state>;

    /// The type of the query data's state
    type State;

    /// Add and check the required world access for this query data
    fn access(access: &mut WorldAccess);

    /// Get the state of the query data for a given archetype
    fn create_state(components: &[TypeId]) -> Option<Self::State>;

    /// Query the components for this data type from the world
    unsafe fn query<'world, 'state>(
        archetype: usize,
        index: usize,
        world: &'world UnsafeWorldCell,
        state: &'state Self::State,
    ) -> Self::Item<'world, 'state>;
}
