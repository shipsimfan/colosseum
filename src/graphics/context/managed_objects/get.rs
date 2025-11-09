use crate::graphics::{CameraInner, context::ManagedGraphicsObjects};
use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

impl ManagedGraphicsObjects {
    /// Get the list of cameras in the system
    pub fn cameras<'a>(&'a self) -> Ref<'a, Vec<Rc<RefCell<CameraInner>>>> {
        self.cameras.borrow()
    }
}
