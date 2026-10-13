use crate::render::frame_graph::Arena;
use std::{
    ops::{Index, IndexMut},
    slice::SliceIndex,
};

impl<'a, T, I: SliceIndex<[T]>> Index<I> for Arena<'a, T> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.data[index]
    }
}

impl<'a, T, I: SliceIndex<[T]>> IndexMut<I> for Arena<'a, T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.data[index]
    }
}
