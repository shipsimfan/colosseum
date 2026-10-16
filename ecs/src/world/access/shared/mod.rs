use std::any::TypeId;

mod new;
mod read_resource;
mod write_resource;

/// The shared access patterns a system has to the world
pub struct SharedWorldAccess {
    /// The resources that are being read by the system
    read_resources: Vec<TypeId>,

    /// The resources that are being written by the system
    write_resources: Vec<TypeId>,
}
