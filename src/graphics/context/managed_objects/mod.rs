use crate::graphics::CameraInner;
use std::{cell::RefCell, rc::Rc};

mod create;
mod get;
mod new;

/// The objects which are created by the game but managed by the engine
pub(in crate::graphics::context) struct ManagedGraphicsObjects {
    /// The cameras that have been registered
    cameras: Rc<RefCell<Vec<Rc<RefCell<CameraInner>>>>>,
}
