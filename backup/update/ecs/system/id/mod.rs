use crate::update::{SystemPhase, ecs::System};
use alexandria::Id;

mod display;
mod get;
mod new;

/// The ID of a system in the ECS
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SystemId {
    /// The phase that the system is in
    phase: SystemPhase,

    /// The index of the system
    id: Id<System>,
}
