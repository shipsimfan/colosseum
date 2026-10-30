use crate::render::FrameHostDataBuffer;
use std::ops::{Index, IndexMut};

impl<T> Index<usize> for FrameHostDataBuffer<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(
            index < self.count,
            "index out of bounds: the len is {} but the index is {}",
            self.count,
            index
        );

        &self.memory[index]
    }
}

impl<T> IndexMut<usize> for FrameHostDataBuffer<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(
            index < self.count,
            "index out of bounds: the len is {} but the index is {}",
            self.count,
            index
        );

        &mut self.memory[index]
    }
}
