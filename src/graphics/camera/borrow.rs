use crate::graphics::{Camera, CameraInner};
use std::cell::{Ref, RefMut};

impl Camera {
    /// Get immutable access to the camera
    pub fn borrow<'a>(&'a self) -> Ref<'a, CameraInner> {
        self.camera.borrow()
    }

    /// Get mutable access to the camera
    pub fn borrow_mut<'a>(&'a self) -> RefMut<'a, CameraInner> {
        self.camera.borrow_mut()
    }
}
