use crate::graphics::{Camera, CameraInner};
use std::{cell::RefCell, rc::Rc};

impl Camera {
    /// Create a new [`Camera`]
    pub(in crate::graphics) fn new(
        camera_list: Rc<RefCell<Vec<Rc<RefCell<CameraInner>>>>>,
        camera: Rc<RefCell<CameraInner>>,
    ) -> Self {
        Camera {
            camera_list,
            camera,
        }
    }
}
