use std::any::{Any, TypeId};

mod get;
mod new;
mod remove;
mod replace;

/// A slot for a resource of a specific type in the ECS system
pub(in crate::resources) struct ResourceSlot {
    /// The type of the contained resource
    r#type: TypeId,

    /// The actual resource stored in this slot
    resource: Option<Box<dyn Any>>,
}
