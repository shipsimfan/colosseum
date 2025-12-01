use std::{cell::RefCell, rc::Rc};

mod inner;

mod borrow;
mod drop;
mod new;

pub use inner::PointLightInner;

/// A light that is located at a specific point and shines outwards equally in all directions
pub struct PointLight {
    /// The list of current point lights that contains this light
    point_light_list: Rc<RefCell<(Vec<Rc<RefCell<PointLightInner>>>, bool)>>,

    /// The reference to the point light itself
    point_light: Rc<RefCell<PointLightInner>>,
}
