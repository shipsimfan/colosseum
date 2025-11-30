use std::{cell::RefCell, rc::Rc};

mod inner;

mod borrow;
mod drop;
mod new;

pub use inner::DirectionalLightInner;

/// A light that is located infinitely far away in a certain direction
pub struct DirectionalLight {
    /// The list of current directional lights that contains this light
    directional_light_list: Rc<RefCell<(Vec<Rc<RefCell<DirectionalLightInner>>>, bool)>>,

    /// The reference to the directional light itself
    directional_light: Rc<RefCell<DirectionalLightInner>>,
}
