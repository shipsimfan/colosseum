use crate::graphics::context::ManagedGraphicsObjects;
use std::{cell::RefCell, rc::Rc};

impl ManagedGraphicsObjects {
    /// Creates a new empty set of [`ManagedGraphicsObjects`]
    pub fn new() -> Self {
        ManagedGraphicsObjects {
            cameras: Rc::new(RefCell::new(Vec::new())),
        }
    }
}
