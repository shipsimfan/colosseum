use crate::graphics::{Material, MaterialInner};
use std::cell::{Ref, RefMut};

impl Material {
    /// Get immutable access to the material
    pub fn borrow<'a>(&'a self) -> Ref<'a, MaterialInner> {
        self.material.borrow()
    }

    /// Get mutable access to the material
    pub fn borrow_mut<'a>(&'a self) -> RefMut<'a, MaterialInner> {
        self.material.borrow_mut()
    }
}
