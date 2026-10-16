use std::any::{Any, TypeId};

mod insert;
mod remove;
mod system_param;

/// Deferred commands to be executed at the end of the phase
pub struct Commands {
    /// The list of resources to insert at the end of the phase
    resources_to_insert: Vec<(TypeId, Box<dyn Any>)>,

    /// The list of resources to remove at the end of the phase
    resources_to_remove: Vec<TypeId>,
}
