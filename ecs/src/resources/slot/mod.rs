use std::{
    any::{Any, TypeId},
    cell::RefCell,
};

mod get;
mod new;
mod remove;
mod replace;

/// A slot for a resource of a specific type in the ECS system
pub(in crate::resources) struct ResourceSlot {
    /// The type of the contained resource
    r#type: TypeId,

    /// The actual resource stored in this slot
    resource: Option<RefCell<Box<dyn Any>>>,
}
