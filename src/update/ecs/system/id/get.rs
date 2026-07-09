use crate::update::{SystemId, SystemPhase, ecs::System};
use alexandria::Id;

impl SystemId {
    /// The phase that the system is in
    pub(in crate::update::ecs) fn phase(&self) -> SystemPhase {
        self.phase
    }

    /// The index of the system
    pub(in crate::update::ecs) fn id(&self) -> Id<System> {
        self.id
    }
}
