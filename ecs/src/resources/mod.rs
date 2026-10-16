use slot::*;

mod get;
mod insert;
mod new;
mod remove;
mod reserve;

mod slot;

/// The set of resources in the ECS system
pub struct Resources {
    /// The currently registered resources in the ECS system
    resources: Vec<ResourceSlot>,
}
