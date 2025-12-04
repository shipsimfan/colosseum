use crate::graphics::util::ConstantBuffer;
use std::ops::{Deref, DerefMut};

impl<T: Sized + Copy> Deref for ConstantBuffer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

impl<T: Sized + Copy> DerefMut for ConstantBuffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.dirty = true;
        &mut self.content
    }
}
