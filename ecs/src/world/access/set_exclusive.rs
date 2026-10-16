use crate::WorldAccess;

impl WorldAccess {
    /// Set the world access to be exclusive, panicking if it is incompatible
    pub fn set_exclusive(&mut self) {
        match self {
            WorldAccess::None => *self = WorldAccess::Exclusive,
            WorldAccess::Shared(_) => {
                panic!(
                    "cannot have exclusive access to a world while it is shared, do you have a `&mut World` with other parameters in a system?"
                )
            }
            WorldAccess::Exclusive => {
                panic!(
                    "cannot have exclusive access to a world while it already has exclusive access, do you have two `&mut World` parameters in a system?"
                )
            }
        }
    }
}
