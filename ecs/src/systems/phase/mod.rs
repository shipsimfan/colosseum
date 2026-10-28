use crate::System;

mod name;

mod add;
mod clear;
mod new;
mod register_archetype;
mod run;

pub use name::*;

/// A single phase of systems that can be run together
pub(in crate::systems) struct Phase {
    /// The systems in the phase
    systems: Vec<Box<dyn System>>,

    /// The systems in the phase which have deferred actions
    deferred: Vec<usize>,

    /// The systems which need archetypes registered
    archetypes_needed: Vec<usize>,
}
