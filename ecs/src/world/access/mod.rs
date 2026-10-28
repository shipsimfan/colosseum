mod shared;

mod get;
mod read_component;
mod read_resource;
mod set_exclusive;
mod write_component;
mod write_resource;

pub use shared::SharedWorldAccess;

/// The type of access that a system needs to the world
#[derive(Debug, Clone)]
pub enum WorldAccess {
    /// No access to the world
    None,

    /// The world can be shared between multiple systems simultaneously
    Shared(SharedWorldAccess),

    /// Exclusive access to the world, no other systems can access it simultaneously
    Exclusive,
}
