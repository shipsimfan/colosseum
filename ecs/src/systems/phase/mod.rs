use crate::System;

mod add;
mod clear;
mod new;
mod run;

/// A single phase of systems that can be run together
pub(in crate::systems) struct Phase {
    /// The systems in the phase
    systems: Vec<System>,
}
