use crate::{UnsafeWorldCell, World};

impl<'world> UnsafeWorldCell<'world> {
    /// Get a reference to the underlying [`World`] instance
    pub unsafe fn get(&self) -> &'world World {
        unsafe { self.world.as_ptr().as_ref().unwrap() }
    }

    /// Get a mutable reference to the underlying [`World`] instance
    pub unsafe fn get_mut(&self) -> &'world mut World {
        unsafe { self.world.as_ptr().as_mut().unwrap() }
    }
}
