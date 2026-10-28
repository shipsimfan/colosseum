use crate::{SharedWorldAccess, WorldAccess};

impl WorldAccess {
    /// Marks a component as being read by the system
    pub fn read_component<Component: 'static>(&mut self) {
        match self {
            WorldAccess::None => {
                let mut shared_access = SharedWorldAccess::new();
                shared_access.read_component::<Component>();
                *self = WorldAccess::Shared(shared_access);
            }
            WorldAccess::Shared(shared_access) => shared_access.read_component::<Component>(),
            WorldAccess::Exclusive => panic!(
                "Cannot read component \"{}\" while having exclusive access, do you have a `&mut World` in the same system?",
                std::any::type_name::<Component>()
            ),
        }
    }
}
