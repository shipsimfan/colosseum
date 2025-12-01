use std::{cell::RefCell, rc::Rc};

mod inner;

mod borrow;
mod drop;
mod new;

pub use inner::SpotLightInner;

/// A light that is located at a specific point and shines outwards in a specific direction
pub struct SpotLight {
    /// The list of current spot lights that contains this light
    spot_light_list: Rc<RefCell<(Vec<Rc<RefCell<SpotLightInner>>>, bool)>>,

    /// The reference to the spot light itself
    spot_light: Rc<RefCell<SpotLightInner>>,
}
