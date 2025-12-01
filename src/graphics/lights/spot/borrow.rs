use crate::graphics::lights::{SpotLight, SpotLightInner};
use std::cell::{Ref, RefMut};

impl SpotLight {
    /// Get immutable access to the spot light
    pub fn borrow<'a>(&'a self) -> Ref<'a, SpotLightInner> {
        self.spot_light.borrow()
    }

    /// Get mutable access to the spot light
    pub fn borrow_mut<'a>(&'a self) -> RefMut<'a, SpotLightInner> {
        self.spot_light.borrow_mut()
    }
}
