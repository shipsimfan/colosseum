use std::{cell::RefCell, rc::Rc};

mod inner;
mod projection;

mod borrow;
mod drop;
mod new;

pub use inner::CameraInner;
pub use projection::CameraProjection;

/// A camera which represents a point of view to render from
#[derive(Clone)]
pub struct Camera {
    /// The list of current cameras that contains this cameraa
    camera_list: Rc<RefCell<Vec<Rc<RefCell<CameraInner>>>>>,

    /// The reference to the camera itself
    camera: Rc<RefCell<CameraInner>>,
}
