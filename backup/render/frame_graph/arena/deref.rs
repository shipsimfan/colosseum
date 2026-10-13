use crate::render::frame_graph::Arena;
use std::ops::{Deref, DerefMut};

impl<'a, T> Deref for Arena<'a, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<'a, T> DerefMut for Arena<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}
