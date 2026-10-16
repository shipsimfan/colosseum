use crate::{UnsafeWorldCell, World, WorldAccess};

/// A parameter that can be used by a system
pub trait SystemParam {
    /// The type of the system parameter when retrieved
    type Item<'world, 'state>;

    /// The state associated with this system parameter
    type State;

    /// Indicates whether the system parameter is deferred
    const DEFERRED: bool = false;

    /// Creates a new state associated with this system parameter
    fn create_state(world: &mut World) -> Self::State;

    /// Add and check the required world access for this system parameter
    fn access(access: &mut WorldAccess);

    /// Retrieves the system parameter from the given resources and archetypes
    unsafe fn retrieve<'world, 'state>(
        state: &'state mut Self::State,
        world: &'world UnsafeWorldCell,
    ) -> Self::Item<'world, 'state>;

    /// Applies the system parameter to the given world
    #[allow(unused_variables)]
    fn apply(state: &mut Self::State, world: &mut World) {}
}
