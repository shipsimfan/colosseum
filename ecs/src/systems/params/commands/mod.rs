use crate::{PhaseName, System};
use std::any::{Any, TypeId};

mod clear;
mod insert;
mod register;
mod remove;
mod system_param;

/// Deferred commands to be executed at the end of the phase
pub struct Commands {
    /// The list of resources to remove at the end of the phase
    resources_to_remove: Vec<TypeId>,

    /// The list of resources to insert at the end of the phase
    resources_to_insert: Vec<(TypeId, Box<dyn Any>)>,

    /// The list of phases to clear at the end of the phase
    phases_to_clear: Vec<Box<dyn PhaseName>>,

    /// The list of system to register at the end of the phase
    systems_to_register: Vec<(Box<dyn PhaseName>, Box<dyn System>)>,
}
