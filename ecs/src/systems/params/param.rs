use crate::{Resources, UnsafeWorldCell, WorldAccess};

/// A parameter that can be used by a system
pub trait SystemParam {
    /// The type of the system parameter when retrieved
    type Item<'world>;

    /// The state associated with this system parameter
    type State;

    /// Creates a new state associated with this system parameter
    fn create_state(resources: &mut Resources) -> Self::State;

    /// Add and check the required world access for this system parameter
    fn access(access: &mut WorldAccess);

    /// Retrieves the system parameter from the given resources and archetypes
    unsafe fn retrieve<'world>(
        world: &'world UnsafeWorldCell,
        state: &Self::State,
    ) -> Self::Item<'world>;
}
