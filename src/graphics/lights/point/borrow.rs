use crate::graphics::lights::{PointLight, PointLightInner};
use std::cell::{Ref, RefMut};

impl PointLight {
    /// Get immutable access to the point light
    pub fn borrow<'a>(&'a self) -> Ref<'a, PointLightInner> {
        self.point_light.borrow()
    }

    /// Get mutable access to the point light
    pub fn borrow_mut<'a>(&'a self) -> RefMut<'a, PointLightInner> {
        self.point_light.borrow_mut()
    }
}
