use crate::{SharedWorldAccess, WorldAccess};

impl WorldAccess {
    /// Marks a component as being written by the system
    pub fn write_component<Component: 'static>(&mut self) {
        match self {
            WorldAccess::None => {
                let mut shared_access = SharedWorldAccess::new();
                shared_access.write_component::<Component>();
                *self = WorldAccess::Shared(shared_access);
            }
            WorldAccess::Shared(shared_access) => shared_access.write_component::<Component>(),
            WorldAccess::Exclusive => panic!(
                "Cannot write component \"{}\" while having exclusive access, do you have a `&mut World` in the same system?",
                std::any::type_name::<Component>()
            ),
        }
    }
}
