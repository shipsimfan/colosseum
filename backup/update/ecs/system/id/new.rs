use crate::update::{SystemId, SystemPhase, ecs::system::System};
use alexandria::Id;

impl SystemId {
    /// Create a new [`SystemId`]
    pub(in crate::update::ecs) fn new(phase: SystemPhase, id: Id<System>) -> SystemId {
        SystemId { phase, id }
    }
}
