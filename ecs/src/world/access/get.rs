use crate::WorldAccess;

impl WorldAccess {
    /// Does this system access archetypes?
    pub fn archetypes_needed(&self) -> bool {
        match self {
            WorldAccess::None => false,
            WorldAccess::Shared(shared) => shared.archetypes_needed(),
            WorldAccess::Exclusive => false, // Exclusive doesn't need to register archetypes
        }
    }

    /// Is a resource accessed by this system?
    pub fn resource<Resource: 'static>(&self) -> bool {
        match self {
            WorldAccess::None => false,
            WorldAccess::Shared(shared) => shared.resource::<Resource>(),
            WorldAccess::Exclusive => true,
        }
    }
}
