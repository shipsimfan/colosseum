mod params;
mod phase;
mod system;

mod clear;
mod get;
mod new;
mod register;
mod register_archetype;
mod run;

pub use params::*;
pub use phase::*;
pub use system::*;

/// The collection of all systems in the ECS
pub(crate) struct Systems {
    /// The phases of systems available to be run
    phases: Vec<(Box<dyn PhaseName>, Phase)>,
}
