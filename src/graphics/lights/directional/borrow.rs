use crate::graphics::lights::{DirectionalLight, DirectionalLightInner};
use std::cell::{Ref, RefMut};

impl DirectionalLight {
    /// Get immutable access to the directional light
    pub fn borrow<'a>(&'a self) -> Ref<'a, DirectionalLightInner> {
        self.directional_light.borrow()
    }

    /// Get mutable access to the directional light
    pub fn borrow_mut<'a>(&'a self) -> RefMut<'a, DirectionalLightInner> {
        self.directional_light.borrow_mut()
    }
}
