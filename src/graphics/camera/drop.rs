use crate::graphics::Camera;
use std::rc::Rc;

impl Drop for Camera {
    fn drop(&mut self) {
        // Is this the last reference to the camera, other than from the list?
        if Rc::strong_count(&self.camera) == 2 {
            // If so, remove it from the list
            let mut camera_list = self.camera_list.borrow_mut();
            let mut index = None;
            for (i, camera) in camera_list.iter().enumerate() {
                if Rc::ptr_eq(camera, &self.camera) {
                    index = Some(i);
                }
            }

            if let Some(i) = index {
                camera_list.swap_remove(i);
            }
        }
    }
}
