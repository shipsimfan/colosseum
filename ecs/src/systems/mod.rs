use phase::*;
use std::any::TypeId;

mod params;
mod phase;
mod system;

mod clear;
mod get;
mod new;
mod register;
mod remove;
mod run;

pub use params::*;
pub use system::*;

/// The collection of all systems in the ECS
pub(crate) struct Systems {
    /// The phases of systems available to be run
    phases: Vec<(TypeId, Phase)>,
}
