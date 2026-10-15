use crate::System;

mod name;

mod add;
mod clear;
mod new;
mod run;

pub use name::*;

/// A single phase of systems that can be run together
pub(in crate::systems) struct Phase {
    /// The systems in the phase
    systems: Vec<Box<dyn System>>,
}
