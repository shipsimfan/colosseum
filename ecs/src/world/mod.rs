use crate::{Archetypes, Resources, Systems};

mod access;
mod r#unsafe;

mod clear;
mod get;
mod insert;
mod new;
mod register;
mod remove;
mod run;

pub use access::*;
pub use r#unsafe::*;

/// A representation of the world in the ECS system
pub struct World {
    /// The resources in the ECS system
    resources: Resources,

    /// The entities and archetypes in the ECS system
    archetypes: Archetypes,

    /// The systems in the ECS system
    systems: Systems,
}
