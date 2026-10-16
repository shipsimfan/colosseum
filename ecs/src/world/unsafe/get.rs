use crate::{UnsafeWorldCell, World};

impl UnsafeWorldCell {
    /// Get a reference to the underlying [`World`] instance
    pub unsafe fn get(&self) -> &World {
        unsafe { self.world.as_ptr().as_ref().unwrap() }
    }

    /// Get a mutable reference to the underlying [`World`] instance
    pub unsafe fn get_mut(&self) -> &mut World {
        unsafe { self.world.as_ptr().as_mut().unwrap() }
    }
}
