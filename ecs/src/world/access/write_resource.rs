use crate::{SharedWorldAccess, WorldAccess};

impl WorldAccess {
    /// Marks a resource as being written by the system
    pub fn write_resource<Resource: 'static>(&mut self) {
        match self {
            WorldAccess::None => {
                let mut shared_access = SharedWorldAccess::new();
                shared_access.write_resource::<Resource>();
                *self = WorldAccess::Shared(shared_access);
            }
            WorldAccess::Shared(shared_access) => shared_access.write_resource::<Resource>(),
            WorldAccess::Exclusive => panic!(
                "Cannot write resource \"{}\" while having exclusive access, do you have a `&mut World` in the same system?",
                std::any::type_name::<Resource>()
            ),
        }
    }
}
