use slot::*;

mod new;

mod slot;

/// The set of resources in the ECS system
pub(crate) struct Resources {
    /// The currently registered resources in the ECS system
    resources: Vec<ResourceSlot>,
}
