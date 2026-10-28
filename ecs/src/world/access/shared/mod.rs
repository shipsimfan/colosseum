use std::any::TypeId;

mod get;
mod new;
mod read_component;
mod read_resource;
mod write_component;
mod write_resource;

/// The shared access patterns a system has to the world
#[derive(Debug, Clone)]
pub struct SharedWorldAccess {
    /// The resources that are being read by the system
    read_resources: Vec<TypeId>,

    /// The resources that are being written by the system
    write_resources: Vec<TypeId>,

    /// The components that are read by the system
    read_components: Vec<TypeId>,

    /// The components that are written by the system
    write_components: Vec<TypeId>,
}
