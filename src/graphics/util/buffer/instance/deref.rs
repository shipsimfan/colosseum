use crate::graphics::util::InstanceBuffer;
use std::ops::{Deref, DerefMut};

impl<T: Sized + Copy> Deref for InstanceBuffer<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

impl<T: Sized + Copy> DerefMut for InstanceBuffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.dirty = true;
        &mut self.content
    }
}
